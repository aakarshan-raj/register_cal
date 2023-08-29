use iced::widget::{column, row, Button, Container, Text, TextInput, Rule};
use iced::{executor, window, Application, Command, Element, Settings, Theme, Length};

#[derive(Default)]
struct Model {
    input_value_one: String,
    input_value_two: String,
    answer: String,
}
#[derive(Debug, Clone)]
pub enum Message {
    Sub,
    Add,
    FirstValue(String),
    SecondValue(String),
    AnswerValue(String),
}

impl Application for Model {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();
    fn new(_flag: Self::Flags) -> (Self, Command<Message>) {
        (
            Model {
                input_value_one: "".to_string(),
                input_value_two: "".to_string(),
                answer: "answer".to_string(),
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("x64 Intel Register Calculator")
    }
    fn view(&self) -> Element<Message> {
        let heading: Element<Message> = Text::new("Register Calculator".to_string())
        .width(Length::Fill)
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .size(20)
        .into();
        let add_button: Element<Message> = Button::new("ADD").on_press(Message::Add).into();
        let sub_button: Element<Message> = Button::new("SUB").on_press(Message::Sub).into();

        let input_three: Element<Message> = TextInput::new("Answer", &self.answer)
                         .on_input(Message::AnswerValue)
                         .into();
      
        let input_one: Element<Message> =
            TextInput::new("R1 Register", &self.input_value_one.to_string())
                .on_input(Message::FirstValue)
                .into();
        let input_two: Element<Message> =
            TextInput::new("R2 Register", &self.input_value_two.to_string())
                .on_input(Message::SecondValue)
                .into();
        //labels
        let register: Element<Message> = Text::new("Register").into();
        let r1: Element<Message> = Text::new("R1").into();
        let r2: Element<Message> = Text::new("R2").into();
        let operation: Element<Message> = Text::new("Operation ").into();
        let result: Element<Message> = Text::new("Result ").into();

        let input_row_one = row!(r1,input_one).spacing(5).padding([10,0,10,0]);
        let input_row_two = row!(r2,input_two).spacing(5).padding([10,0,10,0]);
        let button_row = row!(add_button, sub_button).padding([10,0,10,0]).spacing(20);

        let down_col_one = column!(
            register,
            input_row_one,
            input_row_two,
            operation,
            button_row
        ).width(Length::Fixed(350.0)).padding(20).spacing(10);
        let down_col_two = row!(result, input_three).padding([100,30,0,10]).spacing(10);
        let vert_rule:Element<Message> = Rule::vertical(0).into();
        let down_row = row!(down_col_one, vert_rule,down_col_two);
        let hori_rule:Element<Message> = Rule::horizontal(4).into();
        let col_main = column!(heading,hori_rule, down_row);
        Container::new(col_main).into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Add => {
                self.answer = format!(
                    "{:x}",
                    hex_calculation(
                        1,
                        self.input_value_one.clone(),
                        self.input_value_two.clone()
                    )
                )
            }
            Message::Sub => {
                self.answer = format!(
                    "{:x}",
                    hex_calculation(
                        0,
                        self.input_value_one.clone(),
                        self.input_value_two.clone()
                    )
                )
            }
            Message::FirstValue(value_one) => self.input_value_one = value_one.parse().unwrap(),
            Message::SecondValue(value_two) => self.input_value_two = value_two.parse().unwrap(),
            Message::AnswerValue(value_three) => self.answer = value_three.parse().unwrap(),
        }
        Command::none()
    }
}

fn main() -> Result<(), iced::Error> {
    let my_settings: Settings<()> = iced::settings::Settings {
        window: window::Settings {
            size: (700, 300),
            position: (window::Position::Centered),
            visible: (true),
            resizable: (false),
            decorations: (true),
            transparent: (true),
            ..Default::default()
        },
        ..Default::default()
    };

    Model::run(my_settings)
}

fn hex_calculation(flag: u8, x: String, y: String) -> u64 {
    let first = u64::from_str_radix(&x, 16).unwrap_or_else(|_err|{ return 0 ; });
    let second = u64::from_str_radix(&y, 16).unwrap_or_else(|_err|{ return 0 ; });

    if flag == 1 {
        return first.wrapping_add(second);
    } else {
        return first.wrapping_sub(second);
    }
}
