// pub mod gui;
pub mod md_gen;
pub mod reader;
pub mod yaml_parser;

// use std::collections::HashMap;
// use std::sync::{mpsc, Arc, Mutex};
// use std::thread;
//
// use yaml_rust2::Yaml;
//
// pub fn process_data(excel_path: String,
//                     yaml_path: String,
//                     output_format: String,
//                     progress_sender: mpsc::Sender<f32>)
//                     -> Result<(), Box<dyn std::error::Error>>
// {
//   // Read Excel and YAML files
//   let rows = reader::read_excel(&excel_path)?;
//   let schema = yaml_parser::parse_yaml_schema(&yaml_path)?;
//
//   // Extract fields and map to columns
//   let mut fields = Vec::new();
//   yaml_parser::extract_fields(&schema, "", &mut fields);
//   let headers = &rows[0];
//   let field_map =
//     field_mapper::map_fields_to_columns(&fields, headers);
//
//   let data_rows =
// rows.into_iter().skip(1).collect::<Vec<_>>();   let total_rows
// = data_rows.len() as f32;   let processed_rows =
// Arc::new(Mutex::new(0));
//
//   // Use channel for progress updates
//   let (sender, receiver) = mpsc::channel();
//
//   // Process data rows concurrently
//   let threads: Vec<_> =
//     data_rows.into_iter()
//              .map(|data_row| {
//                let schema = schema.clone();
//                let field_map = field_map.clone();
//                let output_format = output_format.clone();
//                let processed_rows =
// Arc::clone(&processed_rows);                let sender =
// sender.clone();
//
//                thread::spawn(move || {
//                  let mut markdown = String::new();
//
// markdown_generator::generate_markdown(&schema,
// &data_row,
// &field_map,
// 1,                                                        "",
//                                                        &mut
// markdown);
//
//                  // Generate filename
//                  let filename = if let Some(&col_index) =
//                    field_map.get("organization.series_id")
//                  {
//                    output_format.replace("{series_id}",
//                                          &data_row[col_index])
//                  } else {
//                    "output.md".to_string()
//                  };
//
//                  if let Err(e) = std::fs::write(&filename,
// markdown) {                    eprintln!("Failed to write file
// '{}': {}",                              filename, e);
//                  }
//
//                  // Update progress
//                  let mut count =
// processed_rows.lock().unwrap();                  *count += 1;
//                  let progress = *count as f32 / total_rows;
//                  if let Err(e) = sender.send(progress) {
//                    eprintln!("Failed to send progress update:
// {}", e);                  }
//                })
//              })
//              .collect();
//
//   // Handle progress updates
//   for progress in receiver {
//     if let Err(e) = progress_sender.send(progress) {
//       eprintln!("Failed to send progress update to GUI: {}",
// e);     }
//   }
//
//   // Wait for all threads to complete
//   for handle in threads {
//     handle.join().expect("Thread panicked");
//   }
//
//   Ok(())
// }
