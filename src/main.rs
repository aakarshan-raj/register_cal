use std::error::Error;

use iced::widget::{Text, Row, Container};
use iced::{Element, Settings, Application, executor, Command,Theme, Length};
use iced::alignment::{Horizontal,Vertical};

#[derive(Default)]
struct Model;
#[derive(Debug, Clone, Copy)]
pub enum Message {
  
}

impl Application for Model {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();
    fn new(Flag:Self::Flags)->(Self,Command<Message>){
        (Model,Command::none())
    }
    fn title(&self) -> String {
        String::from("x64 Intel Register Calculator")
    }
     fn view(&self) -> Element<Message> {
       let Heading = Text::new("Register Calculator".to_string()).into();
       let row = Row::with_children(vec![Heading]);

       Container::new(row)
       .align_x(Horizontal::Center)
       .width(Length::Fill)
       .into()
    }

     fn update(&mut self, _message: Message)->Command<Message> {
          todo!()
    }
}

fn main()->Result<(),iced::Error>{
      Model::run(Settings::default())
}