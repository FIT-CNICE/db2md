#[cfg(test)]
mod tests
{
  use db2md::md_gen::*;
  use std::collections::HashMap;

  #[test]
  fn test_generate_markdown()
  {
    let mut fields_map: HashMap<String, usize> = HashMap::new();
    fields_map.insert("product.name".to_string(), 0usize);
    fields_map.insert("product.color".to_string(), 1usize);
    fields_map.insert("date".to_string(), 2usize);
    fields_map.insert("customer.price".to_string(), 3usize);
    fields_map.insert("customer.price.origin".to_string(), 4usize);
    let data_row: Vec<String> = vec!["Apple".to_string(),
                                     "red".to_string(),
                                     "2024-10-11".to_string(),
                                     "$3.14".to_string(),
                                     "Produced in California".to_string(),];
    let mut output = String::new();
    let expected_output = String::from("# product\n\n## name\n\nApple\n\n## color\n\nred\n\n# \
                                        date\n\n2024-10-11\n\n# customer\n\n## price\n\n$3.14\n\n### \
                                        origin\n\nProduced in California\n\n");
    generate_markdown(&data_row, &fields_map, &mut output);
    assert_eq!(output, expected_output);
  }
}
