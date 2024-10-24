pub mod gui;
pub mod md_gen;
pub mod reader;
pub mod yaml_parser;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn write_row_to_md(row: &Vec<String>,
                       field_map: &HashMap<String, usize>,
                       file_idx: usize,
                       output_dir: &String,
                       md_prefix: &String)
                       -> usize
{
  let mut md_string = String::new();
  md_gen::generate_markdown(row, field_map, &mut md_string);
  // generate filename
  let output_path = std::path::Path::new(output_dir);
  if !output_path.exists() {
    if let Err(e) = std::fs::create_dir_all(output_path) {
      eprintln!("Failed to create directory '{}': {}",
                output_dir, e);
      return 0usize;
    }
  }
  let filename =
    format!("{}/{}-{:03}.md", output_dir, md_prefix, file_idx);
  // write md file
  if let Err(e) = std::fs::write(&filename, md_string) {
    eprintln!("Failed to write file '{}': {}", filename, e);
    return 0usize;
  }
  1usize
}

pub fn process_data(excel_path: &str,
                    yaml_path: &str,
                    // Using String for md_prefix allows for
                    // ownership and mutability,
                    // which is necessary when the value needs to be
                    // cloned for each thread.
                    // When `md_prefix` is of type `&str`, it's a
                    //    borrowed reference with a limited
                    //    lifetime.
                    // We're trying to clone this reference and
                    //    move it into a new thread.
                    // Rust can't guarantee that the original
                    //    string will live long enough for all
                    //    threads to use it safely.
                    md_prefix: &String,
                    has_header: &bool,
                    progress: &Arc<Mutex<f32>>)
                    -> Result<(), Box<dyn std::error::Error>>
{
  // Read Excel and YAML files
  let mut rows: Vec<Vec<String>> = vec![];
  let meta = reader::read_excel(excel_path, &mut rows)?;
  let tot_row_num = meta.1 as f32;
  let schema = yaml_parser::parse_yaml_schema(yaml_path)?;

  // Extract fields and map to columns
  let mut fields = Vec::new();
  yaml_parser::extract_fields(&schema, "", &mut fields);

  let mut headers = &Vec::new();

  if *has_header {
    headers = &rows[0];
  }

  let mut invalids = vec![];

  let field_map = yaml_parser::map_fields_to_columns(&fields,
                                                     headers,
                                                     &mut invalids);

  // Process data rows concurrently
  let processed_rows = Arc::new(Mutex::new(0));
  let threads: Vec<_> =
    rows.into_iter()
        .enumerate()
        .map(|(idx, row)| {
          let field_map = field_map.clone();
          let md_prefix = md_prefix.clone();
          let processed_rows = Arc::clone(&processed_rows);
          let progress = Arc::clone(progress);
          thread::spawn(move || {
            // generate md string
            let mut md_string = String::new();
            md_gen::generate_markdown(&row,
                                      &field_map,
                                      &mut md_string);
            // generate filename
            let filename = format!("{}-{:03}.md", md_prefix, idx);
            // write md file
            if let Err(e) = std::fs::write(&filename, md_string) {
              eprintln!("Failed to write file '{}': {}", filename, e);
            }
            // update progress
            let mut count = processed_rows.lock().unwrap();
            *count += 1;
            let mut progress_val = progress.lock().unwrap();
            *progress_val = *count as f32 / tot_row_num * 100.0;
          })
        })
        .collect();

  // Wait for all threads to complete
  for handle in threads {
    handle.join().expect("Thread panicked");
  }

  Ok(())
}

// Unit test for the process_data function
#[cfg(test)]
mod tests
{
  use super::*;
  use std::sync::{Arc, Mutex};

  #[test]
  fn test_process_data()
  {
    let excel_path = "./tests/fruit_test.xlsx".to_string();
    let yaml_path = "./tests/schema.yaml".to_string();
    let md_prefix = "ccms-doc".to_string();
    let has_header = false;
    let progress = Arc::new(Mutex::new(0.0));

    let result = process_data(&excel_path,
                              &yaml_path,
                              &md_prefix,
                              &has_header,
                              &progress);
    let progress_val = progress.lock().unwrap();
    assert_eq!(*progress_val, 100.0);
    assert!(result.is_ok());
  }
}
