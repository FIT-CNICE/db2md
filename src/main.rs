#![allow(non_snake_case)]
use db2md::gui::XlsxConverterApp;
use iced::{Application, Settings};

fn main() -> iced::Result
{
  XlsxConverterApp::run(Settings::default())
}
