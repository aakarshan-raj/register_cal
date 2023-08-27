use iced::widget::{Container, Row, Text, TextInput, Button, column};
use iced::{executor, Application, Command, Element, Settings, Theme};

#[derive(Default)]
struct Model {
    input_value_one: i32,
    input_value_two: i32,
    answer: i32,
}
#[derive(Debug, Clone)]
pub enum Message {
    Sub,
    Add,
    FirstValue(String),
    SecondValue(String),
}

impl Application for Model {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();
    fn new(_flag: Self::Flags) -> (Self, Command<Message>) {
        (
            Model {
                input_value_one: 0,
                input_value_two: 0,
                answer: 0,
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("x64 Intel Register Calculator")
    }
    fn view(&self) -> Element<Message> {
        let heading:Element<Message> = Text::new("Register Calculator".to_string()).into();
        let row_one = Row::with_children(vec![heading]);
        let add_button: Element<Message> = Button::new("ADD").on_press(Message::Add).into();
        let sub_button: Element<Message> = Button::new("SUB").on_press(Message::Sub).into();
        let row_two = Row::with_children(vec![add_button, sub_button])
            .spacing(80)
            .padding(10);
        let text: Element<Message> = Text::new(self.answer.to_string()).into();
        let input_one: Element<Message> =
            TextInput
            ::new("R1 Register", &self.input_value_one.to_string())
            .on_input(Message::FirstValue)
            .into();
        let input_two: Element<Message> =
            TextInput
            ::new("R2 Register", &self.input_value_two.to_string())
            .on_input(Message::SecondValue)
            .into();

        let col = column!(row_one, input_one, row_two, input_two, text);
        Container::new(col).into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Add => self.answer = self.input_value_one + self.input_value_two,
            Message::Sub => self.answer = self.input_value_one - self.input_value_two,
            Message::FirstValue(value_one)=>{self.input_value_one = value_one.parse().unwrap()},
            Message::SecondValue(value_two)=>{self.input_value_two = value_two.parse().unwrap()},
        }
        Command::none()
    }
}

fn main() -> Result<(), iced::Error> {
    Model::run(Settings::default())
}
