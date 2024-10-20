use iced::{
  widget::{
    button, column, container, progress_bar, row, text, text_input,
  },
  Element, Fill,
};

use std::thread;

use crate::process_data;

#[derive(Default)]
pub struct Db2Md
{
  // GUI State
  excel_path: String,
  yaml_path: String,
  output_prefix: String,
  progress: f32,
}

#[derive(Debug, Clone)]
pub enum Message
{
  SelectExcel(String),
  SelectYaml(String),
  FormatChanged(String),
  StartConversion,
  UpdateProgress(f32),
}

impl Db2Md
{
  fn update(&mut self,
            message: Message)
  {
    use Message::*;
    match message {
      SelectExcel(s) => self.excel_path = s,
      SelectYaml(s) => self.yaml_path = s,
      FormatChanged(s) => self.output_prefix = s,
      UpdateProgress(n) => self.progress = n * 100.0,
      _ => {}
    }
  }

  fn view(state: &Db2Md) -> Element<Message>
  {
    container(column!["Welcome to Db2Md",
                      row!["Select Excel FIle",].spacing(10),
                      "Bottom"].spacing(10)).padding(10)
                                            .center_x(Fill)
                                            .center_y(Fill)
                                            .into()
  }
}
