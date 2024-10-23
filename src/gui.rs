use iced::theme::Theme;
use iced::widget::{
  button, checkbox,
  checkbox::Icon,
  column, container, progress_bar, row, text,
  text::{LineHeight, Shaping},
  text_input, Svg,
};
use iced::{Alignment, Element, Font, Length};
use iced::{Application, Command};
use rfd::AsyncFileDialog;

// State management
#[derive(Debug)]
pub struct Db2MdApp
{
  selected_file: Option<String>,
  has_header: bool,
  file_prefix: String,
  progress: f32,
  rows_loaded: Option<usize>,
  data_matrix: Vec<Vec<String>>,
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

impl Default for Db2MdApp
{
  fn default() -> Self
  {
    Self { selected_file: None,
           has_header: false,
           file_prefix: String::from("ccms-doc"),
           progress: 0.0,
           rows_loaded: None,
           data_matrix: Vec::new(),
           is_loading: false }
  }
}

impl Application for Db2MdApp
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
    String::from("db2md")
  }

  fn theme(&self) -> iced::Theme
  {
    iced::Theme::CatppuccinMacchiato
    // or
    // iced::Theme::Light
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

    let path_text = if let Some(path) = self.selected_file.clone() {
      text(format!("{} selected", path))
    } else {
      text("Nothing selected")
    };

    let file_selection =
      row![button("Select XLSX file").on_press(Message::SelectFile),
           path_text,
           button("Load").on_press(Message::LoadFile)].width(Length::Fill).align_items(Alignment::Center);

    let rows_info = if let Some(rows) = self.rows_loaded {
      text(format!("Loaded {} rows", rows))
    } else if self.is_loading {
      text("Loading...")
    } else {
      text("Shown how many rows loaded once loading(sync) is \
            complete")
    };

    let header_selection = row![text("Has header?"),
           checkbox("Yes", self.has_header).on_toggle(|_| {
                                   Message::SetHasHeader(true)
                                 }).icon(Icon {
                    font: Font::DEFAULT,
                    code_point: '*',
                    size: None,
                    line_height: LineHeight::default(),
                    shaping: Shaping::default()
                }),
           checkbox("No", !self.has_header).on_toggle(|_| {
                                  Message::SetHasHeader(false)
                                 }).icon(Icon {
                    font: Font::DEFAULT,
                    code_point: '*',
                    size: None,
                    line_height: LineHeight::default(),
                    shaping: Shaping::default()
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
