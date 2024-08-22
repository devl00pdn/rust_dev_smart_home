use iced::{Sandbox, Settings, Size};
use iced::alignment::Alignment;
use iced::Element;
use iced::widget::{Button, Column, Row, Text};
use iced::window;

use crate::socket_lib_wrapper::SocketLibWrapper;

mod socket_lib_wrapper;
#[derive(Default)]
struct SocketWidget {
    socket: Option<SocketLibWrapper>,
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
            let is_socket_enable = socket.current_state();

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

            let pwr = socket.power_consumption_wt();
            let pwr_txt = format!("Power: {} wt", pwr);

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
        if let Some(s) = &self.socket {
            if s.current_state() {
                s.turn_off();
            } else {
                s.turn_on();
            }
        }
    }

    fn handle_connection_pressed(&mut self) {
        if self.socket.is_none() {
            let socket = SocketLibWrapper::new("127.0.0.1:55331".to_string()).expect("Oh no!");
            self.socket = Some(socket);
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