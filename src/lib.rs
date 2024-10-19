// pub mod gui;
pub mod md_gen;
pub mod reader;
pub mod yaml_parser;

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub fn process_data(excel_path: &str,
                    yaml_path: &str,
                    has_header: bool,
                    progress_sender: mpsc::Sender<f32>)
                    -> Result<(), Box<dyn std::error::Error>>
{
  // Read Excel and YAML files
  let rows = reader::read_excel(&excel_path)?;
  let tot_row_num = rows.len() as f32;
  let schema = yaml_parser::parse_yaml_schema(&yaml_path)?;

  // Extract fields and map to columns
  let mut fields = Vec::new();
  yaml_parser::extract_fields(&schema, "", &mut fields);

  let headers = &Vec::new();

  if has_header {
    let headers = &rows[0];
  }

  let field_map =
    yaml_parser::map_fields_to_columns(&fields, headers);

  // process data rows iteratively
  // let processed_rows = Arc::new(Mutex::new(0));
  // let (sender, receiver) = mpsc::channel();
  let threads: Vec<_> =
    rows.into_iter()
        .enumerate()
        .map(|(idx, row)| {
          // let field_map = field_map.clone();
          // let processed_rows = Arc::clone(&processed_rows);
          // let sender = sender.clone();
          // thread::spawn(move || {
          // generate md string
          let mut md_string = String::new();
          md_gen::generate_markdown(&row, &field_map, &mut md_string);
          // generate filename
          let filename = format!("ccms-doc-{:03}.md", idx);
          // write md file
          if let Err(e) = std::fs::write(&filename, md_string) {
            eprintln!("Failed to write file '{}': {}", filename, e);
          }
          // update progress
          // let mut count = processed_rows.lock().unwrap();
          // *count += 1;
          // let progress = *count as f32 / tot_row_num;
          // if let Err(e) = sender.send(progress) {
          //   eprintln!("Failed to send progress update: {}",
          // e); }
          // })
        })
        .collect();

  // handle progress updates
  // for progress in receiver {
  //   if let Err(e) = progress_sender.send(progress) {
  //     eprintln!("Failed to send progress update to GUI: {}", e);
  //   }
  // }

  // wait for all threads to complete
  // for handle in threads {
  //   handle.join().expect("Thread panicked");
  // }

  Ok(())
}

// unit test for the process_data function
#[cfg(test)]
mod tests
{
  use super::*;
  use std::sync::mpsc;

  #[test]
  fn test_process_data()
  {
    let excel_path = "./tests/fruit_test.xlsx";
    let yaml_path = "./tests/schema.yaml";
    let has_header = false;
    let (progress_sender, _progress_receiver) = mpsc::channel();

    let result = process_data(excel_path,
                              yaml_path,
                              has_header,
                              progress_sender);
    assert!(result.is_ok());
  }
}
