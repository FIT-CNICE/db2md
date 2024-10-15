use std::error::Error;
use std::fs::File;
use std::io::Read;
use yaml_rust2::{Yaml, YamlLoader};

pub fn parse_yaml_schema(file_path: &str)
                         -> Result<Yaml, Box<dyn Error>>
{
  let mut file = File::open(file_path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let docs = YamlLoader::load_from_str(&contents)?;
  Ok(docs[0].clone())
}

/// Recursively extracts field paths from the YAML schema.
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
