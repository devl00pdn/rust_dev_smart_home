use std::io;
use std::ops::Deref;

use thiserror::Error;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

use crate::client_tokio::{read_srt, send_str};
use crate::errors::{ConnectError, ConnectResult, RecvResult, SendResult};

pub struct ServerStp {
    tcp: TcpListener,
}

impl ServerStp {
    pub async fn bind<Addr>(addr: Addr) -> BindResult
    where
        Addr: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addr).await?;
        Ok(Self { tcp })
    }

    pub async fn incoming(&self) -> ConnectResult<StpConnection> {
        let (s, _) = self.tcp.accept().await?;
        Self::try_handshake(s).await
    }

    pub async fn try_handshake(mut stream: TcpStream) -> ConnectResult<StpConnection> {
        let handshake_req_msg = read_srt(&mut stream).await.map_err(|e| ConnectError::BadHandshake(e.to_string()))?;
        if !handshake_req_msg.eq(crate::protocol::HANDSHAKE_REQUEST) {
            return Err(ConnectError::BadHandshake("Handshake request not matched".to_string()));
        }
        let _ = send_str(&mut stream, crate::protocol::HANDSHAKE_RESPOND).await;
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
    pub async fn send_response<Resp: AsRef<str>>(&mut self, response: Resp) -> SendResult {
        send_str(&mut self.stream, response).await
    }

    pub async fn revc_request(&mut self) -> RecvResult {
        read_srt(&mut self.stream).await
    }
}

impl Deref for StpConnection {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        &self.stream
    }
}