use iced::theme::Theme;
use iced::widget::{
  button, checkbox, column, container, progress_bar, row, text,
  text_input, Svg,
};
use iced::{Alignment, Element};
use iced::{Application, Command};
use rfd::AsyncFileDialog;

// State management
#[derive(Debug, Default)]
pub struct XlsxConverterApp
{
  selected_file: Option<String>,
  has_header: bool,
  file_prefix: String,
  progress: f32,
  rows_loaded: Option<usize>,
  is_loading: bool,
}

// Messages for user interactions and async operations
#[derive(Debug, Clone)]
pub enum Message
{
  SelectFile,
  FileSelected(Option<String>),
  LoadFile,
  SetHasHeader(bool),
  SetFilePrefix(String),
  Convert,
  UpdateProgress(f32),
  RowsLoaded(usize),
}

impl Application for XlsxConverterApp
{
  type Message = Message;
  type Theme = Theme;
  type Executor = iced::executor::Default;
  type Flags = ();

  fn new(_flags: ()) -> (Self, Command<Message>)
  {
    (Self::default(), Command::none())
  }

  fn title(&self) -> String
  {
    String::from("XLSX Converter")
  }

  fn update(&mut self,
            message: Message)
            -> Command<Message>
  {
    match message {
      Message::SelectFile => {
        // Launch file dialog
        Command::perform(async {
                           AsyncFileDialog::new()
                            .add_filter("Excel", &["xlsx"])
                            .pick_file()
                            .await
                            .map(|file| file.path().to_string_lossy().into_owned())
                         },
                         Message::FileSelected)
      }
      Message::FileSelected(path) => {
        self.selected_file = path;
        Command::none()
      }
      Message::LoadFile => {
        self.is_loading = true;
        // Simulate file loading - replace with actual xlsx
        // loading
        Command::none()
      }
      Message::SetHasHeader(value) => {
        self.has_header = value;
        Command::none()
      }
      Message::SetFilePrefix(value) => {
        self.file_prefix = value;
        Command::none()
      }
      Message::Convert => {
        self.progress = 0.0;
        Command::none()
      }
      Message::UpdateProgress(value) => {
        self.progress = value;
        Command::none()
      }
      Message::RowsLoaded(rows) => {
        self.rows_loaded = Some(rows);
        self.is_loading = false;
        Command::none()
      }
    }
  }

  fn view(&self) -> Element<Message>
  {
    let header = Svg::from_path("./assets/header.svg");

    let file_selection = row![
            button("Select XLSX file (rfd)").on_press(Message::SelectFile),
            button("Load").on_press(Message::LoadFile)
        ]
        .spacing(10);

    let rows_info = if let Some(rows) = self.rows_loaded {
      text(format!("Loaded {} rows", rows))
    } else if self.is_loading {
      text("Loading...")
    } else {
      text("Shown how many rows loaded once loading(sync) is \
            complete")
    };

    let header_selection = row![text("Has header?"),
           checkbox("Yes", false).on_toggle(|_| {
                                   Message::SetHasHeader(true)
                                 }),
           checkbox("No", false).on_toggle(|_| {
                                  Message::SetHasHeader(false)
                                })].spacing(10)
                                   .align_items(Alignment::Center);

    let prefix_input = row![
            text("Prefix for generated files"),
            text_input("Text input", &self.file_prefix)
                .on_input(Message::SetFilePrefix)
                .padding(10)
        ].spacing(10)
                       .align_items(Alignment::Center);

    let progress = row![
            progress_bar(0.0..=1.0, self.progress),
            button("Convert").on_press(Message::Convert)
        ]
        .spacing(10)
        .align_items(Alignment::Center);

    container(column![header,
                      file_selection,
                      rows_info,
                      header_selection,
                      prefix_input,
                      progress,].spacing(20)
                                .padding(20)).center_x()
                                             .center_y()
                                             .into()
  }
}
