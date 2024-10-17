use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use yaml_rust2::{Yaml, YamlLoader};

pub fn parse_yaml_schema(file_path: &str)
                         -> Result<Yaml, Box<dyn Error>>
{
  let contents = read_to_string(file_path)?;
  let docs = YamlLoader::load_from_str(&contents)?;
  Ok(docs[0].clone())
}

// Recursively extracts field paths from the YAML schema.
pub fn extract_fields(yaml: &Yaml,
                      prefix: &str,
                      fields: &mut Vec<String>)
{
  match yaml {
    Yaml::Hash(hash) => {
      for (key, value) in hash {
        if let Yaml::String(key_str) = key {
          let new_prefix = if prefix.is_empty() {
            key_str.clone()
          } else {
            format!("{}.{}", prefix, key_str)
          };
          extract_fields(value, &new_prefix, fields);
        }
      }
    }
    Yaml::String(_) => {
      fields.push(prefix.to_string());
    }
    _ => {}
  }
}

pub fn map_fields_to_columns(fields: &[String],
                             headers: &[String])
                             -> HashMap<String, usize>
{
  let mut mapping = HashMap::new();
  if headers.is_empty() {
    for (index, field) in fields.iter().enumerate() {
      mapping.insert(field.clone(), index);
    }
  } else {
    for field in fields {
      let field_name = field.split('.').last().unwrap_or(field);
      if let Some(index) =
        headers.iter().position(|h| h == field_name)
      {
        mapping.insert(field.clone(), index);
      } else {
        println!("Warning: Field '{}' not found in Excel headers",
                 field_name);
      }
    }
  }
  mapping
}
