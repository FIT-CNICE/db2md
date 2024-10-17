#[cfg(test)]
mod tests
{
  use db2md::md_gen::*;
  use std::collections::HashMap;
  use yaml_rust2::YamlLoader;

  #[test]
  #[ignore]
  fn test_generate_markdown()
  {
    let yaml_str = "
        organization:
          sbu: text
          product: text
        date: date
        ";
    let docs = YamlLoader::load_from_str(yaml_str).unwrap();
    let schema = &docs[0];

    let data_row = vec!["SBU_Value".to_string(),
                        "Product_Value".to_string(),
                        "Date_Value".to_string(),];
    let mut field_map = HashMap::new();
    field_map.insert("organization.sbu".to_string(), 0);
    field_map.insert("organization.product".to_string(), 1);
    field_map.insert("date".to_string(), 2);

    let mut output = String::new();
    generate_markdown(schema,
                      &data_row,
                      &field_map,
                      1,
                      "",
                      &mut output);

    let expected_output = "organization
sbu

SBU_Value
product

Product_Value
date

Date_Value

";
    assert_eq!(output, expected_output);
  }
}
