#![allow(unused)]
/// refer to https://github.com/RGGH/iced_tutorial/tree/main
use controller::con;
mod controller;

use iced::widget::image::Handle;
use iced::{
    alignment::{Horizontal, Vertical},
    font::Family,
    widget::{
        button, column, container, pick_list, text, text::Shaping, text_input, Button, Column,
        Container, Image, Renderer, Row, Text, Theme,
    },
    window, Alignment, Color, Font, Length, Point, Sandbox, Settings, Size,
};
use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;

fn main() -> iced::Result {
    MyApp::run(Settings {
        window: window::Settings {
            size: Size {
                width: 520.,
                height: 420.,
            },
            position: window::Position::Specific(Point { x: 900., y: 120. }),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

//pub enum Status {
//Active,
//Hovered,
//Pressed,
//Disabled,
//}

#[derive(Debug, Clone)]
enum MyAppMessage {
    ButtonPressed,
    UpdateIP(String),
    UpdateUser(String),
    Update3(String),
    DoNothing,
}

#[derive(Debug, Clone)]
enum Message {
    ClearStatus,
    TextInputChanged(String),
}

struct MyApp {
    status: String,
    clear_button: button::State,
    text_ip: String,
    text_user: String,
    pick_list_3: Option<String>,
}

impl Sandbox for MyApp {
    type Message = MyAppMessage;

    fn new() -> Self {
        MyApp {
            status: String::new(),
            clear_button: button::State::new(),
            text_ip: "".to_string(),
            text_user: "".to_string(),
            pick_list_3: Some("Choose a host".into()),
        }
    }
    fn theme(&self) -> iced::Theme {
        iced::Theme::Dracula
    }

    fn title(&self) -> String {
        String::from("Guissh - SSH Client")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            MyAppMessage::ButtonPressed => {
                //Status::Pressed;
                con(&self.text_ip, &self.text_user)
            }
            MyAppMessage::UpdateIP(s) => {
                self.text_ip = s;
            }
            MyAppMessage::UpdateUser(s) => {
                self.text_user = s;
            }
            MyAppMessage::Update3(s) => self.pick_list_3 = Some(s),
            MyAppMessage::DoNothing => {}
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let image: iced::widget::Image<Handle> = Image::new("resources/ferris.png")
            .width(Length::Fill)
            .height(Length::Fill);

        let background: iced::widget::Container<'_, Self::Message, Theme, Renderer> =
            Container::new(image)
                .width(Length::Fill)
                .height(Length::Fill);
        let status_bar = Text::new(&self.status).width(iced::Length::Fill).size(20);

        let value = "Some text";

        column![
            container("").padding(20).align_x(Horizontal::Center),
            container(button("Connect").on_press(MyAppMessage::ButtonPressed))
                .width(Length::Fill)
                .align_x(Horizontal::Center),
            Row::new().push(background),
            Row::new().padding(10),
            container(pick_list(
                ["192.168.1.12:22", "192.168.1.7:22", "10.10.10.33:22"]
                    .map(|s| s.to_string())
                    .to_vec(),
                self.pick_list_3.clone(),
                |s| MyAppMessage::UpdateIP(s)
            ))
            .align_x(Horizontal::Center)
            .width(Length::Fill),
            Row::new().padding(30).push(status_bar),
            text("Enter username").style(Color::from_rgb(1., 0.6, 0.2)),
            text_input("username", self.text_user.as_str()).on_input(MyAppMessage::UpdateUser),
            text("Enter IP address:port number").style(Color::from_rgb(1., 0.6, 0.2)),
            text("eg 192.168.1.12:22").style(Color::from_rgb(1., 0.9, 0.2)),
            text_input("ip address:port", self.text_ip.as_str()).on_input(MyAppMessage::UpdateIP),
        ]
        .into()
    }
}
