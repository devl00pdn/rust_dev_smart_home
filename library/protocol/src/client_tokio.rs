use std::io;
use std::io::ErrorKind;
use std::ops::Deref;
use std::pin::Pin;

use thiserror::Error;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};

use crate::errors::{ConnectResult, RecvError, RecvResult, SendError, SendResult};
use crate::protocol;

#[derive(Debug)]
pub struct ClientStp {
    stream: Pin<Box<TcpStream>>,
}

impl ClientStp {
    async fn handshake(mut stream: Pin<Box<TcpStream>>) -> ConnectResult<Self> {
        let msg = protocol::handshake_request_msg();
        stream.write_all(msg.as_bytes()).await?;
        let mut buff = [0; 256];
        let _n = stream.read(&mut buff).await?;
        let resp_raw_mgs = String::from_utf8(Vec::from(buff)).expect("Error to convert msg to string");
        if !resp_raw_mgs.contains(protocol::handshake_respond_msg().as_str()) {
            "Unexpected response".to_string();
        }
        Ok(Self { stream })
    }

    pub async fn connect<Addr>(addr: Addr) -> ConnectResult<Self>
    where
        Addr: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addr).await?;
        let stream = Pin::new(Box::new(stream));
        Self::handshake(stream).await
    }

    pub async fn send_request<Data: AsRef<str>>(&mut self, msg: Data) -> RequestResult {
        send_str(&mut self.stream, msg).await?;
        let resp = read_srt(&mut self.stream).await?;
        Ok(resp)
    }
}

impl Deref for ClientStp {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        &self.stream
    }
}


pub async fn send_str<Writer: AsyncWrite + Unpin, Data: AsRef<str>>(writer: &mut Writer, msg: Data) -> SendResult {
    let coded_msg = protocol::wrap_message(msg);
    writer.write_all(coded_msg.as_bytes()).await?;
    Ok(())
}

pub async fn read_srt<Reader: AsyncRead + Unpin>(reader: &mut Reader) -> RecvResult {
    let mut buff: Vec<u8> = vec![0; 1024];
    let rlen = reader.read(&mut buff).await?;
    if rlen == 0 {
        return Err(RecvError::from(io::Error::from(ErrorKind::BrokenPipe)));
    }
    let raw_str = String::from_utf8(buff).expect("error utf8 to str");
    return match protocol::unwrap_message(&raw_str).map_err(RecvError::Other) {
        Ok(msgs) => {
            match msgs.len() {
                2.. => Ok(msgs.iter().map(|v| v.to_string() + ",").collect::<String>()),
                1 => Ok(msgs[0].clone()),
                _ => Err(RecvError::BadEncoding)
            }
        }
        Err(e) => { Err(e) }
    };
}


pub type RequestResult = Result<String, RequestError>;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error(transparent)]
    Send(#[from] SendError),
    #[error(transparent)]
    Recv(#[from] RecvError),
}