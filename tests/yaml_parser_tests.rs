#[cfg(test)]
mod tests
{
  use db2md::yaml_parser::*;
  use yaml_rust2::YamlLoader;

  #[test]
  fn test_parse_yaml_schema_from_file()
  {
    let yaml_file = "./tests/schema.yaml";
    if let Ok(schema) = parse_yaml_schema(yaml_file) {
      assert!(schema["organization"].is_hash());
      assert!(schema["date"].to_owned()
                            .into_string()
                            .unwrap()
                            .is_ascii());
    } else {
      println!("error reading yaml file");
      assert!(false)
    }
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
