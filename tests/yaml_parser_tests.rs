#[cfg(test)]
mod tests
{
  use db2md::reader::*;
  use db2md::yaml_parser::*;
  use yaml_rust2::YamlLoader;

  #[test]
  fn test_parse_yaml_schema_from_file()
  {
    let yaml_file = "./tests/schema.yaml";
    if let Ok(schema) = parse_yaml_schema(yaml_file) {
      assert_eq!(schema["organization"]["sbu"].as_str().unwrap(),
                 "text");
      assert_eq!(schema["date"].as_str().unwrap(), "date");
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

  #[test]
  fn test_field_map_no_header()
  {
    let yaml_file = "./tests/schema.yaml";
    if let Ok(schema) = parse_yaml_schema(yaml_file) {
      let mut fields = Vec::new();
      extract_fields(&schema, "", &mut fields);
      let headers = Vec::new();
      let mut invalids = vec![];
      let hmap =
        map_fields_to_columns(&fields, &headers, &mut invalids);
      assert_eq!(hmap["organization.sbu"], 0usize);
      assert_eq!(hmap["organization.product"], 1usize);
      assert_eq!(hmap["date"], 2usize);
      assert_eq!(hmap["complaint.price"], 3usize);
      assert_eq!(hmap["complaint.customer"], 4usize);
    } else {
      assert!(false);
    }
  }

  #[test]
  fn test_field_map_with_header()
  {
    let xslx = "./tests/fruit_test.xlsx";
    let mut sheet = vec![];
    read_excel(xslx, &mut sheet).unwrap();
    let headers = &sheet[0];
    let fields = vec!["product.APPLE".to_string(),
                      "product.color.red".to_string(),
                      "2024-10-11".to_string(),
                      "312".to_string(),
                      "California".to_string()];
    let mut invalids = vec![];
    let hmap = map_fields_to_columns(&fields, headers, &mut invalids);
    assert_eq!(hmap["product.APPLE"],
               headers.iter()
                      .position(|h| h.as_str() == "APPLE")
                      .unwrap());
    assert_eq!(hmap["product.color.red"],
               headers.iter()
                      .position(|h| h.as_str() == "red")
                      .unwrap());
    assert_eq!(hmap["2024-10-11"],
               headers.iter()
                      .position(|h| h.as_str() == "2024-10-11")
                      .unwrap());
    assert!(headers.iter()
                   .position(|h| h.as_str() == "312")
                   .is_none());
  }
}
