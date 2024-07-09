use std::io;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::ops::Deref;
use std::time::Duration;

use thiserror::Error;

use crate::errors::{ConnectResult, RecvError, RecvResult, SendError, SendResult};
use crate::protocol;

#[derive(Debug)]
pub struct ClientStp {
    stream: TcpStream,
}

impl ClientStp {
    fn handshake(mut stream: TcpStream) -> ConnectResult<Self> {
        let msg = protocol::handshake_request_msg();
        stream.write_all(msg.as_bytes())?;
        stream.set_read_timeout(Some(Duration::from_secs(1)))?;
        let mut buff = [0; 256];
        let _n = stream.read(&mut buff)?;
        let resp_raw_mgs = String::from_utf8(Vec::from(buff)).expect("Error to convert msg to string");
        if !resp_raw_mgs.contains(protocol::handshake_respond_msg().as_str()) {
            "Unexpected response".to_string();
        }
        Ok(Self { stream })
    }

    pub fn connect<Addr>(addr: Addr) -> ConnectResult<Self>
    where
        Addr: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addr)?;
        Self::handshake(stream)
    }

    pub fn send_request<Data: AsRef<str>>(&mut self, msg: Data) -> RequestResult {
        send_str(&self.stream, msg)?;
        let resp = read_srt(&self.stream)?;
        Ok(resp)
    }
}

impl Deref for ClientStp {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        &self.stream
    }
}


pub fn send_str<Writer: Write, Data: AsRef<str>>(mut writer: Writer, msg: Data) -> SendResult {
    let coded_msg = protocol::wrap_message(msg);
    writer.write_all(coded_msg.as_bytes())?;
    Ok(())
}

pub fn read_srt<Reader: Read>(mut reader: Reader) -> RecvResult {
    let mut buff: Vec<u8> = vec![0; 1024];
    let rlen = reader.read(&mut buff)?;
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