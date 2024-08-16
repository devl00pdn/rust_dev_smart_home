use std::cell::RefCell;
use std::net::SocketAddr;
use std::time;

use iced::{Sandbox, Settings, Size};
use iced::alignment::Alignment;
use iced::Element;
use iced::widget::{Button, Column, Row, Text};
use iced::window;

use smart_home_lib::common::traits::device::{PowerConsumptionMeter, Switchable};
use smart_home_lib::devices::socket_tcp::socket_thread::SocketTcpWrapper;

#[derive(Default)]
struct SocketWidget {
    socket: Option<RefCell<SocketTcpWrapper>>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    PowerTogglePressed,
    ConnectionTogglePressed,
}

impl Sandbox for SocketWidget {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Smart Socket")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PowerTogglePressed => {
                self.toggle_socket_state();
            }
            Message::ConnectionTogglePressed => {
                self.handle_connection_pressed();
            }
        }
    }


    fn view(&self) -> Element<Message> {
        let mut pwr_toggle_btn = None;
        let mut state_field = None;
        let mut pwr_field = None;

        if let Some(socket) = &self.socket {
            let mut pwr_txt = "Power: Unknown".to_string();
            let is_socket_enable = socket.borrow_mut().current_state().unwrap();

            let pwr_label = if is_socket_enable {
                "Turn Off"
            } else {
                "Turn On"
            };

            let state_txt = if is_socket_enable {
                "Socket: on".to_string()
            } else {
                "Socket: off".to_string()
            };

            if let Ok(Some(pwr)) = socket.borrow_mut().power_consumption_wt() {
                pwr_txt = format!("Power: {} wt", pwr);
            }
            state_field = Some(Text::new(state_txt));
            pwr_field = Some(Text::new(pwr_txt));
            pwr_toggle_btn = Some(Button::new(pwr_label));
        }

        let connection_label = if self.socket.is_none() {
            "Connect"
        } else {
            "Disconnect"
        };

        let content = Column::new().width(200)
            .push(Row::new()
                .push_maybe(state_field.map(|x| x.size(20))))
            .push(Row::new()
                .push_maybe(pwr_field.map(|x| x.size(20))))
            .push(Row::new().push_maybe(
                pwr_toggle_btn.map(|x| x.on_press(Message::PowerTogglePressed))))
            .push(Row::new().push(Button::new(connection_label)
                .on_press(Message::ConnectionTogglePressed)))
            .align_items(Alignment::Center);
        content.into()
    }
}

impl SocketWidget {
    fn toggle_socket_state(&mut self) {
        if let Some(s) = &mut self.socket {
            if s.borrow_mut().current_state().unwrap() {
                s.borrow_mut().turn_off().unwrap();
            } else {
                s.borrow_mut().turn_on().unwrap();
            }
        }
    }

    fn handle_connection_pressed(&mut self) {
        if self.socket.is_none() {
            let addr: SocketAddr = "127.0.0.1:55331".parse().unwrap();
            let ten_millis = time::Duration::from_millis(200);
            self.socket = match SocketTcpWrapper::new(addr, ten_millis) {
                Ok(s) => {
                    Some(RefCell::new(s))
                }
                Err(_) => { None }
            }
        } else {
            self.socket = None
        }
    }
}
fn main() {
    let settings = Settings {
        window: window::Settings {
            size: Size { width: 200.0, height: 200.0 },
            position: Default::default(),
            min_size: None,
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            level: Default::default(),
            icon: None,
            platform_specific: Default::default(),
            exit_on_close_request: true,
        },
        ..Default::default()
    };
    SocketWidget::run(settings).expect("TODO: panic message")
}