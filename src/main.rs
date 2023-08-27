use std::error::Error;

use iced::mouse::Button;
use iced::widget::{Text, Row, Container, button, column, TextInput, text_input};
use iced::{Element, Settings, Application, executor, Command,Theme, Length};
use iced::alignment::{Horizontal,Vertical};

#[derive(Default)]
struct Model{
    check:i32
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
  Increment,
  Decrement,
}

impl Application for Model {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();
    fn new(Flag:Self::Flags)->(Self,Command<Message>){
        (Model{check:0},Command::none())
    }
    fn title(&self) -> String {
        String::from("x64 Intel Register Calculator")
    }
     fn view(&self) -> Element<Message> {
       let Heading = Text::new("Register Calculator".to_string()).into();
       let row_one = Row::with_children(vec![Heading]);
       let add_button:Element<Message> = button("ADD").on_press(Message::Increment).into();
       let sub_button:Element<Message> = button("SUB").on_press(Message::Decrement).into();
       let row_two = Row::with_children(vec![add_button,sub_button]).spacing(80).padding(10);
       let text:Element<Message> = Text::new(self.check.to_string()).size(50).horizontal_alignment(Horizontal::Center).width(Length::Shrink).into();
       let input_one:Element<Message> = TextInput::new("R1 Register", "").into();
       let input_two:Element<Message> = TextInput::new("R2 Register", "").into();

       let col = column![row_one,input_one,row_two,input_two,text];
       Container::new(col)
       .align_x(Horizontal::Center)
       .width(Length::Fill)
       .into()
       
    }

     fn update(&mut self, message: Message)->Command<Message> {
          match message {
              Message::Decrement=>{self.check -= 1},
              Message::Increment=>{self.check += 1},
          }
          Command::none()
    }
}

fn main()->Result<(),iced::Error>{
      Model::run(Settings::default())
}