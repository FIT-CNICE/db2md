use std::collections::HashMap;
use yaml_rust2::Yaml;

pub fn generate_markdown(yaml: &Yaml,
                         data_row: &[String],
                         field_map: &HashMap<String, usize>,
                         level: usize,
                         prefix: &str,
                         output: &mut String)
{
  match yaml {
    Yaml::Hash(hash) => {
      for (key, value) in hash {
        if let Yaml::String(key_str) = key {
          let heading =
            format!("{} {}\n\n", "#".repeat(level), key_str);
          output.push_str(&heading);
          let new_prefix = if prefix.is_empty() {
            key_str.clone()
          } else {
            format!("{}.{}", prefix, key_str)
          };
          generate_markdown(value,
                            data_row,
                            field_map,
                            level + 1,
                            &new_prefix,
                            output);
        }
      }
    }
    Yaml::String(_) => {
      // It's a field, retrieve data
      if let Some(&col_index) = field_map.get(prefix) {
        let value = &data_row[col_index];
        output.push_str(&format!("{}\n\n", value));
      } else {
        output.push_str("\n\n");
      }
    }
    _ => {}
  }
}
