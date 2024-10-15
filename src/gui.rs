use iced::{
  widget::{button, column, progress_bar, text, text_input},
  Alignment, Element, Length, Settings, Theme,
};

use std::sync::mpsc;
use std::thread;

use crate::process_data;

#[derive(Default)]
pub struct Db2Md
{
  // GUI State
  excel_path: String,
  yaml_path: String,
  output_format: String,
  progress: f32,
  // Widgets State
}

#[derive(Debug, Clone)]
pub enum Message
{
  SelectExcel,
  SelectYaml,
  FormatChanged(String),
  StartConversion,
  UpdateProgress(f32),
}

impl Db2Md
{
  type Message = Message;
  type Theme = Theme;
  type Flags = ();

  pub fn new() -> Self
  {
    Db2Md { excel_path: "".into(),
            yaml_path: "".into(),
            output_format: "{series_id}.md".into(),
            progress: 0.0 }
  }

  pub fn title(&self) -> String
  {
    String::from("db2md")
  }

  pub fn update(&mut self,
                message: Message)
  {
    match message {
      Message::SelectExcel => {
        if let Some(path) =
          rfd::FileDialog::new().add_filter("Excel files", &["xlsx"])
                                .pick_file()
        {
          self.excel_path = path.display().to_string();
        }
      }
      Message::SelectYaml => {
        if let Some(path) =
          rfd::FileDialog::new().add_filter("YAML files",
                                            &["yaml", "yml"])
                                .pick_file()
        {
          self.yaml_path = path.display().to_string();
        }
      }
      Message::FormatChanged(new_format) => {
        self.output_format = new_format;
      }
      Message::StartConversion => {
        if self.excel_path.is_empty() || self.yaml_path.is_empty() {
          // Handle error (e.g., show a message)
          eprintln!("Please select both Excel and YAML files.");
          return;
        }

        let excel_path = self.excel_path.clone();
        let yaml_path = self.yaml_path.clone();
        let output_format = self.output_format.clone();

        // Create a channel for progress updates
        let (progress_sender, progress_receiver) = mpsc::channel();

        // Spawn a thread for processing
        thread::spawn(move || {
          if let Err(e) = process_data(excel_path,
                                       yaml_path,
                                       output_format,
                                       progress_sender)
          {
            eprintln!("Error during processing: {}", e);
          }
        });

        // Spawn a thread to receive progress updates
        let app_handle =
          iced::futures::executor::block_on(async move {
            while let Ok(progress) = progress_receiver.recv() {
              self.progress = progress;
              // Since we are in a separate thread, we need to
              // send a message back to the main thread.
              // In iced 0.9, this can be handled differently.
              // For simplicity, we'll update the progress
              // directly here.
            }
          });
      }
      Message::UpdateProgress(progress) => {
        self.progress = progress;
      }
    }
  }

  pub fn view(&self) -> Element<Message>
  {
    let content = column![
            button("Select Excel File")
                .on_press(Message::SelectExcel)
                .width(Length::Fill),
            text(&self.excel_path)
                .size(16)
                .width(Length::Fill)
                .height(Length::Shrink),
            button("Select YAML Schema")
                .on_press(Message::SelectYaml)
                .width(Length::Fill),
            text(&self.yaml_path)
                .size(16)
                .width(Length::Fill)
                .height(Length::Shrink),
            text_input("Output Format (e.g., {series_id}.md)", &self.output_format)
                .on_input(Message::FormatChanged),
            button("Start Conversion")
                .on_press(Message::StartConversion)
                .width(Length::Fill),
            progress_bar(0.0..=1.0, self.progress)
                .width(Length::Fill)
                .height(Length::Fill),
        ].spacing(10)
                  .padding(20)
                  .align_x(Alignment::Center);

    content.into()
  }

  pub fn theme(&self) -> Theme
  {
    Theme::Dark
  }
}
