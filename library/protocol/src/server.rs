use std::io;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::ops::Deref;

use thiserror::Error;

use crate::errors::{ConnectError, ConnectResult, RecvResult, SendResult};

pub struct ServerStp {
    tcp: TcpListener,
}

impl ServerStp {
    pub fn bind<Addr>(addr: Addr) -> BindResult
        where Addr: ToSocketAddrs {
        let tcp = TcpListener::bind(addr)?;
        Ok(Self { tcp })
    }
    pub fn incoming(&self) -> impl Iterator<Item=ConnectResult<StpConnection>> + '_ {
        self.tcp.incoming().map(|s| {
            match s {
                Ok(s) => Self::try_handshake(s),
                Err(e) => Err(ConnectError::Io(e)),
            }
        })
    }

    pub fn try_handshake(stream: TcpStream) -> ConnectResult<StpConnection> {
        let handshake_req_msg = crate::read_srt(&stream).map_err(|e| ConnectError::BadHandshake(e.to_string()))?;
        if !handshake_req_msg.eq(crate::protocol::HANDSHAKE_REQUEST) {
            return Err(ConnectError::BadHandshake("Handshake request not matched".to_string()));
        }
        let _ = crate::send_str(&stream, crate::protocol::HANDSHAKE_RESPOND);
        Ok(StpConnection { stream })
    }
}


type BindResult = Result<ServerStp, BindError>;

/// Bind to socket error
#[derive(Debug, Error)]
pub enum BindError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub struct StpConnection {
    stream: TcpStream,
}

impl StpConnection {
    pub fn send_response<Resp: AsRef<str>>(&mut self, response: Resp) -> SendResult {
        crate::send_str(&mut self.stream, response)
    }

    pub fn revc_request(&mut self) -> RecvResult {
        crate::read_srt(&mut self.stream)
    }
}

impl Deref for StpConnection {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        &self.stream
    }
}