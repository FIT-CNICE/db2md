#![allow(non_snake_case)]
use db2md::gui::Db2MdApp;
use iced::{Application, Settings};

fn main() -> iced::Result
{
  Db2MdApp::run(Settings::default())
}
