use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::ops::Deref;
use std::time::Duration;

use thiserror::Error;

use crate::errors::{ConnectResult, RecvError, SendError};
use crate::errors::ConnectError::BadHandshake;
use crate::protocol;

pub struct ClientStp {
    stream: TcpStream,
}

impl ClientStp {
    pub fn handshake(mut stream: TcpStream) -> ConnectResult<Self> {
        let msg = protocol::handshake_request_msg();
        stream.write_all(msg.as_bytes())?;
        stream.set_read_timeout(Some(Duration::from_secs(1)))?;
        let mut buff = [0; 256];
        let _n = stream.read(&mut buff)?;
        let resp_raw_mgs = String::from_utf8(Vec::from(buff)).expect("Error to convert msg to string");
        if !resp_raw_mgs.contains(protocol::handshake_respond_msg().as_str()) {
            BadHandshake("Unexpected response".to_string());
        }
        return Ok(Self { stream });
    }

    pub fn connect<Addr>(addr: Addr) -> ConnectResult<Self>
        where Addr: ToSocketAddrs {
        let stream = TcpStream::connect(addr)?;
        Self::handshake(stream)
    }

    pub fn send_request<Data: AsRef<str>>(&mut self, msg: Data) -> RequestResult {
        crate::send_str(&self.stream, msg)?;
        let resp = crate::read_srt(&self.stream)?;
        Ok(resp)
    }
}

impl Deref for ClientStp {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        &self.stream
    }
}


pub type RequestResult = Result<String, RequestError>;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error(transparent)]
    Send(#[from] SendError),
    #[error(transparent)]
    Recv(#[from] RecvError),
}