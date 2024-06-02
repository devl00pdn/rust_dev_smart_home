use regex::Regex;

pub const HANDSHAKE_REQUEST: &str = "hi_server";
pub const HANDSHAKE_RESPOND: &str = "hi_client";
pub const WRAPPING_SYMBOLS: &str = "@@";


//todo:  Добавить кастомные ошибки. сервер и клиент
pub fn handshake_request_msg() -> String {
    wrap_message(HANDSHAKE_REQUEST)
}

pub fn handshake_respond_msg() -> String {
    wrap_message(HANDSHAKE_RESPOND)
}

pub type UMsgResult = Result<Vec<String>, String>;

pub fn unwrap_message(raw_msg: &str) -> UMsgResult {
    // get text1,text2,... from @@text1@@@@text2@@ string
    let re = Regex::new(r#"@@([^@]+)@@"#).unwrap();
    let mut parsed_msgs: Vec<String> = Vec::new();
    for msg_capture in re.captures_iter(raw_msg) {
        let msg_body = msg_capture.get(1).map_or("", |m| m.as_str());
        parsed_msgs.push(msg_body.to_string())
    }
    return if !parsed_msgs.is_empty() {
        Ok(parsed_msgs)
    } else {
        Err("Parsing error".to_string())
    };
}

pub fn wrap_message<Data: AsRef<str>>(msg: Data) -> String {
    let mut wrapped = WRAPPING_SYMBOLS.to_string();
    wrapped.insert_str(0, msg.as_ref().to_string().as_str());
    wrapped.insert_str(0, WRAPPING_SYMBOLS);
    wrapped
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_wrapping() {
        assert_eq!("@@abc@@", wrap_message("abc"));
    }

    #[test]
    fn check_handshake_request_msg() {
        assert_eq!("@@hi_server@@", handshake_request_msg());
    }

    #[test]
    fn check_handshake_resp_msg() {
        assert_eq!("@@hi_client@@", handshake_respond_msg());
    }


    #[test]
    fn check_unwrap_once() {
        if let Ok(msg) = unwrap_message("@@some_text@@") {
            assert_eq!(msg.len(), 1);
            assert_eq!(msg[0], "some_text".to_string());
        } else { panic!() }
    }

    #[test]
    fn check_unwrap_multiple() {
        if let Ok(msg) = unwrap_message("@@some_text1@@@@some_text2@@@@some_text3@@") {
            assert_eq!(msg.len(), 3);
            assert_eq!(msg[0], "some_text1".to_string());
            assert_eq!(msg[1], "some_text2".to_string());
            assert_eq!(msg[2], "some_text3".to_string());
        } else { panic!() }
    }

    #[test]
    fn check_unwrap_corrupted_last() {
        if let Ok(msg) = unwrap_message("@@some_text1@@@@some_text2@@@@some_text3@") {
            assert_eq!(msg.len(), 2);
            assert_eq!(msg[0], "some_text1".to_string());
            assert_eq!(msg[1], "some_text2".to_string());
        } else { panic!() }
    }

    #[test]
    fn check_unwrap_parsing_err() {
        if let Err(msg) = unwrap_message("@some_text1@@") {
            assert_eq!(msg, "Parsing error".to_string());
        }
    }
}