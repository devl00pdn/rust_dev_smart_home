use std::io::{ErrorKind, Read, Write};
use std::io;

use crate::errors::{RecvError, RecvResult, SendResult};

pub mod protocol;
pub mod errors;
pub mod client;
pub mod server;

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
    return match protocol::unwrap_message(&raw_str).map_err(|e| errors::RecvError::Other(e)) {
        Ok(msgs) => {
            if msgs.len() > 1 {
                Ok(msgs.iter().map(|v| v.to_string() + ",").collect::<String>())
            } else if msgs.len() == 1 {
                Ok(msgs[0].clone())
            } else {
                Err(RecvError::BadEncoding)
            }
        }
        Err(e) => { Err(e) }
    };
}