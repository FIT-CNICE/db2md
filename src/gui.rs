#![allow(unused_imports)]
use iced::advanced::image::Bytes;
use iced::advanced::widget::text as advanced_text;
use iced::alignment::Vertical;
use iced::widget::checkbox::Icon;
use iced::widget::image::Handle;
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::{button, checkbox, column, container, image, progress_bar, row, text, text_input, Space};
use iced::{Color, Element, Fill, Font, Length, Task};
use rfd::AsyncFileDialog;

use crate::reader::read_excel;
use crate::write_row_to_md;
use crate::yaml_parser::*;
use iced::futures::{FutureExt as IcedFutureExt, StreamExt as IcedStreamExt};
use std::collections::HashMap;

// State management
#[derive(Debug)]
pub struct Db2MdApp
{
  // data file
  has_header: bool,
  selected_file: Option<String>,
  sheet_name: Option<String>,
  rows_loaded: Option<usize>,
  cols_loaded: Option<usize>,
  data_matrix: Vec<Vec<String>>,
  // schema yaml
  selected_yaml: Option<String>,
  fields_map: HashMap<String, usize>,
  invalid_fields: Vec<String>,
  // I/O
  file_prefix: String,
  output_dir: String,
  progress: usize,
  write_fails: Vec<usize>,
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
  SetOutputDir(String),
  Convert,
  UpdateProgress((usize, usize)),
  RowsLoaded,
}

impl Default for Db2MdApp
{
  fn default() -> Self
  {
    Self { has_header: false,
           selected_file: None,
           selected_yaml: None,
           file_prefix: String::from("ccms-doc"),
           output_dir: String::from("_md"),
           progress: 0,
           sheet_name: None,
           rows_loaded: None,
           cols_loaded: None,
           data_matrix: Vec::new(),
           write_fails: Vec::new(),
           fields_map: HashMap::new(),
           invalid_fields: Vec::new(),
           is_loading: false }
  }
}

impl Db2MdApp
{
  pub fn title(&self) -> String
  {
    String::from("db2md")
  }

  pub fn theme(&self) -> iced::Theme
  {
    iced::Theme::CatppuccinMacchiato
  }

  pub fn update(&mut self,
                message: Message)
                -> Task<Message>
  {
    match message {
      Message::SelectFile => {
        // Launch file dialog
        Task::perform(async {
                        AsyncFileDialog::new().add_filter("Excel", &["xlsx"])
                                              .pick_file()
                                              .await
                                              .map(|file| file.path().to_string_lossy().into_owned())
                      },
                      Message::FileSelected)
      },

      Message::FileSelected(path) => {
        self.selected_file = path;
        Task::none()
      },

      Message::LoadFile => {
        if self.selected_file.is_none() {
          Task::none()
        } else {
          self.is_loading = true;
          // loading
          let mut data = vec![];
          let meta = read_excel(self.selected_file.as_ref().unwrap(), &mut data).unwrap_or((String::from("N/A"), 0, 0));
          self.data_matrix = data;
          self.rows_loaded = Some(meta.1);
          self.cols_loaded = Some(meta.2);
          self.sheet_name = Some(meta.0);
          Task::perform(async {}, |_| Message::RowsLoaded)
        }
      },

      Message::SelectYaml => {
        // Launch file dialog
        Task::perform(async {
                        AsyncFileDialog::new().add_filter("Yaml", &["yaml"])
                                              .pick_file()
                                              .await
                                              .map(|file| file.path().to_string_lossy().into_owned())
                      },
                      Message::YamlSelected)
      },

      Message::YamlSelected(path) => {
        self.selected_yaml = path;
        Task::none()
      },

      Message::LoadYaml => {
        if self.selected_yaml.is_none() {
          Task::none()
        } else {
          // loading
          if let Ok(yml) = parse_yaml_schema(self.selected_yaml.as_ref().unwrap()) {
            let mut fields_map_raw = vec![];
            extract_fields(&yml, "", &mut fields_map_raw);

            let headers = if self.has_header {
              self.data_matrix.first().unwrap()
            } else {
              &Vec::new()
            };
            self.invalid_fields.clear();
            self.fields_map = map_fields_to_columns(fields_map_raw.as_ref(),
                                                    headers,
                                                    &mut self.invalid_fields);
          }
          Task::none()
        }
      },

      Message::SetHasHeader(value) => {
        self.has_header = value;
        Task::none()
      },

      Message::SetFilePrefix(value) => {
        self.file_prefix = value;
        Task::none()
      },

      Message::SetOutputDir(value) => {
        self.output_dir = value;
        Task::none()
      },

      Message::Convert => {
        self.write_fails.clear();
        self.progress = 0;

        // Clone what we need before the async block
        let has_header = self.has_header;
        let fields_map = self.fields_map.clone();
        let output_dir = self.output_dir.clone();
        let file_prefix = self.file_prefix.clone();

        // Create futures for each row
        let all_rows: Vec<_> = self.data_matrix
                                   .iter()
                                   .enumerate()
                                   .map(|(idx, row)| {
                                     let data_row = row.clone();
                                     let fields = fields_map.clone();
                                     let out_dir = output_dir.clone();
                                     let prefix = file_prefix.clone();
                                     let row_num = idx + if has_header { 1usize } else { 0usize };

                                     async move {
                                       let result =
                                         write_row_to_md(&data_row, &fields, row_num, &out_dir, &prefix).await;
                                       (idx, result)
                                     }
                                   })
                                   .collect();

        // Convert Vec of futures into a stream and process with
        // Task::run
        let stream = smol::stream::iter(all_rows).map(|future| future.boxed()) // box it for thread
                                                 // ownership
                                                 .buffer_unordered(8); // Process up to 8 futures
                                                                       // concurrently
                                                                       // .boxed();

        Task::run(stream, Message::UpdateProgress)
      },

      Message::UpdateProgress((idx, result)) => {
        self.progress += 1;
        if result == 0 {
          self.write_fails.push(idx + 1);
        }
        Task::none()
      },

      Message::RowsLoaded => {
        self.is_loading = false;
        Task::none()
      },
    }
  }

  pub fn view(&self) -> Element<Message>
  {
    let png = include_bytes!(".././assets/header.png");
    let png_bytes = Bytes::from_static(png);
    let png_handle = Handle::from_bytes(png_bytes);
    let header = image(png_handle).content_fit(iced::ContentFit::Contain);
    let warn_color = Color::from_rgb(1.0, 0.6, 0.2);

    let path_text = if let Some(path) = self.selected_file.clone() {
      text(path.to_string())
    } else {
      text("Nothing selected")
    };

    let file_selection = row![button("Select XLSX").on_press(Message::SelectFile),
                              Space::with_width(10),
                              path_text,
                              Space::with_width(Length::Fill),
                              button("Load").on_press(Message::LoadFile)].align_y(Vertical::Center)
                                                                         .width(Fill);

    let rows_info = if let Some(rows) = self.rows_loaded {
      let cols = self.cols_loaded.as_ref().unwrap();
      let sheet = self.sheet_name.as_ref().unwrap();
      text(format!("Loaded {} rows of {} strings in {}",
                   // support rendering of complex scripts
                   rows,
                   cols,
                   sheet)).shaping(advanced_text::Shaping::Advanced)
    } else if self.is_loading {
      text("Loading...")
    } else {
      text(" ")
    };

    let yaml_path = if let Some(path) = self.selected_yaml.clone() {
      text(path.to_string())
    } else {
      text("No schema selected")
    };

    let yaml_selection = row![button("Select YAML").on_press(Message::SelectYaml),
                              Space::with_width(10),
                              yaml_path,
                              Space::with_width(Length::Fill),
                              button("Load").on_press(Message::LoadYaml)].align_y(Vertical::Center)
                                                                         .width(Fill);

    let yaml_info = if !self.fields_map.is_empty() {
      let cols = self.cols_loaded.as_ref().unwrap();
      let field_num = self.fields_map.len();
      if field_num > *cols {
        text(format!("Find {} fields but each row has {} columns, only first {} fields will be used",
                     field_num, cols, cols)).color(warn_color)
      } else if field_num < *cols {
        text(format!("Find {} fields but each row has {} columns, only first {} columns will be used",
                     field_num, cols, field_num)).color(warn_color)
      } else {
        text("All fields found in selected yaml will be used to generate MD")
      }
    } else {
      text("No Field Loaded")
    };

    let invalid_field = if !self.invalid_fields.is_empty() {
      text(format!("Invalid fields in Yaml {:?}", self.invalid_fields)).color(warn_color)
    } else if !self.fields_map.is_empty() {
      text("All fields in Yaml are found in the sheet")
    } else {
      text("")
    };

    let header_selection =
      row![text("Has header?"),
           checkbox("Yes", self.has_header).on_toggle(|_| { Message::SetHasHeader(true) })
                                           .icon(Icon { font: Font::DEFAULT,
                                                        code_point: 'x',
                                                        size: None,
                                                        line_height: LineHeight::default(),
                                                        shaping: Shaping::default() }),
           checkbox("No", !self.has_header).on_toggle(|_| { Message::SetHasHeader(false) })
                                           .icon(Icon { font: Font::DEFAULT,
                                                        code_point: 'x',
                                                        size: None,
                                                        line_height: LineHeight::default(),
                                                        shaping: Shaping::default() })].spacing(10)
                                                                                       .align_y(Vertical::Center);

    let prefix_input = row![text("Prefix for generated files"),
                            text_input("Text input", &self.file_prefix).on_input(Message::SetFilePrefix)
                                                                       .padding(10)].spacing(10)
                                                                                    .align_y(Vertical::Center);

    let output_dir = row![text("Output directory for generated files"),
                          text_input("Text input", &self.output_dir).on_input(Message::SetOutputDir)
                                                                    .padding(10)].spacing(10)
                                                                                 .align_y(Vertical::Center);

    let progress = self.progress;
    let percentage: f32 = progress as f32 / self.rows_loaded.unwrap_or(1usize) as f32 * 100f32;
    let progress = row![progress_bar(0.0..=100.0, percentage),
                        button("Convert").on_press(Message::Convert)].spacing(10)
                                                                     .align_y(Vertical::Center);

    let completion_msg = if !self.write_fails.is_empty() {
      text(format!("Fail to write the following rows: {:?}",
                   self.write_fails)).color(warn_color)
    } else {
      text("")
    };

    container(column![header,
                      header_selection,
                      file_selection,
                      rows_info,
                      yaml_selection,
                      yaml_info,
                      invalid_field,
                      prefix_input,
                      output_dir,
                      progress,
                      completion_msg].spacing(20)
                                     .max_width(800)
                                     .padding(10)).center(Fill)
                                                  .into()
  }
}
