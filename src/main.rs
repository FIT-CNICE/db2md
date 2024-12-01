#![allow(non_snake_case)]
use db2md::gui::Db2MdApp;
use iced::application;

fn main() -> iced::Result
{
  application(Db2MdApp::title, Db2MdApp::update, Db2MdApp::view).theme(Db2MdApp::theme)
                                                                .run()
}
