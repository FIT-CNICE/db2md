#[cfg(test)]
mod tests
{
  use std::ascii::AsciiExt;

  use super::*;
  use db2md::yaml_parser::*;
  use yaml_rust2::YamlLoader;

  #[test]
  fn test_parse_yaml_schema()
  {
    let yaml_str = "
        organization:
          sbu: text
          product: text
          series_id: number
        date: date
        engineer: text
        complaint:
          customer: text
          content: text
          status: text
        ";
    let docs = YamlLoader::load_from_str(yaml_str).unwrap();
    let schema = &docs[0];
    assert!(schema["organization"].is_hash());
    assert!(schema["date"].into_string().unwrap().is_ascii());
  }

  #[test]
  fn test_extract_fields()
  {
    let yaml_str = "
        organization:
          sbu: text
          product: text
          series_id: number
        date: date
        engineer: text
        complaint:
          customer: text
          content: text
          status: text
        ";
    let docs = YamlLoader::load_from_str(yaml_str).unwrap();
    let schema = &docs[0];
    let mut fields = Vec::new();
    extract_fields(schema, "", &mut fields);
    assert_eq!(fields.len(), 8);
    assert!(fields.contains(&"organization.sbu".to_string()));
    assert!(fields.contains(&"date".to_string()));
  }
}
