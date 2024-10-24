use iced::theme::Theme;
use iced::widget::{
  button, checkbox,
  checkbox::Icon,
  column, container, progress_bar, row, text,
  text::{LineHeight, Shaping},
  text_input, Space, Svg,
};
use iced::{Alignment, Element, Font, Length};
use iced::{Application, Command};
use rfd::AsyncFileDialog;

use crate::reader::read_excel;
use crate::yaml_parser::*;
use std::collections::HashMap;

// State management
#[derive(Debug)]
pub struct Db2MdApp
{
  selected_file: Option<String>,
  selected_yaml: Option<String>,
  has_header: bool,
  file_prefix: String,
  progress: f32,
  sheet_name: Option<String>,
  rows_loaded: Option<usize>,
  cols_loaded: Option<usize>,
  data_matrix: Vec<Vec<String>>,
  yaml_fields: HashMap<String, usize>,
  is_loading: bool,
}

// Messages for user interactions and async operations
#[derive(Debug, Clone)]
pub enum Message
{
  SelectFile,
  FileSelected(Option<String>),
  LoadFile,
  SelectYaml,
  YamlSelected(Option<String>),
  LoadYaml,
  SetHasHeader(bool),
  SetFilePrefix(String),
  Convert,
  UpdateProgress(f32),
  RowsLoaded,
}

impl Default for Db2MdApp
{
  fn default() -> Self
  {
    Self { selected_file: None,
           selected_yaml: None,
           has_header: false,
           file_prefix: String::from("ccms-doc"),
           progress: 0.0,
           sheet_name: None,
           rows_loaded: None,
           cols_loaded: None,
           data_matrix: Vec::new(),
           yaml_fields: HashMap::new(),
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
        if self.selected_file.is_none() {
          return Command::none();
        } else {
          self.is_loading = true;
          // loading
          let mut data = vec![];
          let meta =
            read_excel(self.selected_file.as_ref().unwrap(),
                       &mut data).unwrap_or((String::from("N/A"),
                                             0,
                                             0));
          self.data_matrix = data;
          self.rows_loaded = Some(meta.1);
          self.cols_loaded = Some(meta.2);
          self.sheet_name = Some(meta.0);
          Command::perform(async {}, |_| Message::RowsLoaded)
        }
      }

      Message::SelectYaml => {
        // Launch file dialog
        Command::perform(async {
                           AsyncFileDialog::new()
                            .add_filter("Yaml", &["yaml"])
                            .pick_file()
                            .await
                            .map(|file| file.path().to_string_lossy().into_owned())
                         },
                         Message::YamlSelected)
      }

      Message::YamlSelected(path) => {
        self.selected_yaml = path;
        Command::none()
      }

      Message::LoadYaml => {
        if self.selected_yaml.is_none() {
          return Command::none();
        } else {
          // loading
          if let Ok(yml) =
            parse_yaml_schema(self.selected_yaml.as_ref().unwrap())
          {
            let mut yaml_fields_raw = vec![];
            extract_fields(&yml, "", &mut yaml_fields_raw);

            let headers = if self.has_header {
              self.data_matrix.first().unwrap()
            } else {
              &Vec::new()
            };
            self.yaml_fields =
              map_fields_to_columns(yaml_fields_raw.as_ref(),
                                    headers);
          }
          return Command::none();
        }
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

      Message::RowsLoaded => {
        self.is_loading = false;
        Command::none()
      }
    }
  }

  fn view(&self) -> Element<Message>
  {
    let header = Svg::from_path("./assets/header.svg");

    let path_text = if let Some(path) = self.selected_file.clone() {
      text(format!("{}", path))
    } else {
      text("Nothing selected")
    };

    let file_selection =
      row![button("Select XLSX file").on_press(Message::SelectFile),
           Space::with_width(10), path_text, Space::with_width(Length::Fill),
           button("Load").on_press(Message::LoadFile)].align_items(Alignment::Center);

    let rows_info = if let Some(rows) = self.rows_loaded {
      let cols = self.cols_loaded.as_ref().unwrap();
      let sheet = self.sheet_name.as_ref().unwrap();
      text(format!("Loaded {} rows of {} strings in {}",
                   rows, cols, sheet))
    } else if self.is_loading {
      text("Loading...")
    } else {
      text(" ")
    };

    let yaml_path = if let Some(path) = self.selected_yaml.clone() {
      text(format!("{}", path))
    } else {
      text("No schema selected")
    };

    let yaml_selection =
      row![button("Select YAML file").on_press(Message::SelectYaml),
           Space::with_width(10), yaml_path, Space::with_width(Length::Fill),
           button("Load").on_press(Message::LoadYaml)].align_items(Alignment::Center);

    let yaml_info = if self.yaml_fields.len() > 0 {
      let cols = self.cols_loaded.as_ref().unwrap();
      let field_num = self.yaml_fields.len();
      if field_num > *cols {
        text(format!("Find {} fields but each row has {} columns, \
                      only first {} fields will be used",
                     field_num, cols, cols))
      } else if field_num < *cols {
        text(format!("Find {} fields but each row has {} columns, \
                      only first {} columns will be used",
                     field_num, cols, field_num))
      } else {
        text("All fields found in selected yaml will be used to \
              generate MD")
      }
    } else {
      text("No Field Loaded")
    };

    let header_selection = row![text("Has header?"),
           checkbox("Yes", self.has_header).on_toggle(|_| {
                                   Message::SetHasHeader(true)
                                 }).icon(Icon {
                    font: Font::DEFAULT,
                    code_point: '\u{2705}',
                    size: None,
                    line_height: LineHeight::default(),
                    shaping: Shaping::default()
                }),
           checkbox("No", !self.has_header).on_toggle(|_| {
                                  Message::SetHasHeader(false)
                                 }).icon(Icon {
                    font: Font::DEFAULT,
                    code_point: '\u{2705}',
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
                      header_selection,
                      file_selection,
                      rows_info,
                      yaml_selection,
                      yaml_info,
                      prefix_input,
                      progress,].spacing(20)
                                .padding(20)).center_x()
                                             .center_y()
                                             .into()
  }
}
