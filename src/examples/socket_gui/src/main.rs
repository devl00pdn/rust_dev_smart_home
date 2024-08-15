use std::net::SocketAddr;

use iced::{Sandbox, Settings};
use iced::alignment::Alignment;
use iced::Element;
use iced::widget::{Button, Column, Row, Text};

use smart_home_lib::common::traits::device::Switchable;
use smart_home_lib::devices::socket_tcp::socket_std::SocketTcp;

#[derive(Default)]
struct SocketWidget {
    socket_enabled: bool,
    power_consumption_wt: f32,
    socket: Option<SocketTcp>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    PowerTogglePressed,
    ConnectionToggle,
}

impl Sandbox for SocketWidget {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PowerTogglePressed => {
                self.toggle_socket_state();
            }
            Message::ConnectionToggle => {
                self.handle_onnection();
            }
        }
    }


    fn view(&self) -> Element<Message> {
        let state = if self.socket_enabled {
            "Socket state: Enabled"
        } else {
            "Socket state: Disable"
        }.to_string();
        let pwr_label = if self.socket_enabled {
            "Turn Off"
        } else {
            "Turn On"
        };
        let connection_label = if self.socket.is_none() {
            "Connect"
        } else {
            "Disconnect"
        };

        let content = Column::new()
            .push(Row::new()
                .push(Text::new(state).size(20))
            )
            .push(Row::new().push(
                Button::new(pwr_label)
                    .on_press(Message::PowerTogglePressed)
            ).push(Button::new(connection_label)
                .on_press(Message::ConnectionToggle)))
            .align_items(Alignment::Center);
        content.into()
    }
}

impl SocketWidget {
    fn toggle_socket_state(&mut self) {
        match &mut self.socket {
            None => {}
            Some(s) => {
                if self.socket_enabled {
                    s.turn_off().unwrap();
                    self.socket_enabled = false;
                } else {
                    s.turn_on().unwrap();
                    self.socket_enabled = true;
                }
            }
        }
    }

    fn handle_onnection(&mut self) {
        if self.socket.is_none() {
            let addr: SocketAddr = "127.0.0.1:55331".parse().unwrap();
            self.socket = match SocketTcp::new(addr) {
                Ok(s) => {
                    Some(s)
                }
                Err(_) => { None }
            }
        } else {
            self.socket = None
        }
    }
}

fn main() {
    // wtf ok()?
    // thread request pwr and state
    // small widget size
    SocketWidget::run(Settings::default()).expect("TODO: panic message")
}