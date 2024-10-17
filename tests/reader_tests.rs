#[cfg(test)]
mod tests
{
  use db2md::reader::*;

  #[test]
  fn test_read_excel()
  {
    let file_path = "./tests/fruit_test.xlsx";

    let range =
      read_excel(file_path).expect("Failed to read Excel file");
    let first_row: Vec<String> = range.rows()
                                      .into_iter()
                                      .next()
                                      .unwrap()
                                      .into_iter()
                                      .map(|d| d.to_string())
                                      .collect();

    assert_eq!(first_row[0], "APPLE");
    assert_eq!(first_row[1], "red");
    assert_eq!(first_row[2], "45576");
    assert_eq!(first_row[3], "3.12");
    assert_eq!(first_row[4], "California");
  }
}
