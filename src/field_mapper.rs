use std::collections::HashMap;

pub fn map_fields_to_columns(fields: &[String],
                             headers: &[String])
                             -> HashMap<String, usize>
{
  let mut mapping = HashMap::new();
  for field in fields {
    let field_name = field.split('.').last().unwrap_or(field);
    if let Some(index) = headers.iter().position(|h| h == field_name)
    {
      mapping.insert(field.clone(), index);
    } else {
      eprintln!("Warning: Field '{}' not found in Excel headers",
                field_name);
    }
  }
  mapping
}
