use iced::widget::{Text, Row, container};
use iced::{Sandbox, Element, Settings, Application, executor, Command,Theme};

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

       container(row).into()
    }

     fn update(&mut self, message: Message)->Command<Message> {
          todo!()
    }
}

fn main(){
    Model::run(Settings::default());
}