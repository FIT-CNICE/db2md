#[cfg(test)]
mod tests
{
  use db2md::reader::*;

  #[test]
  fn test_read_excel()
  {
    let file_path = "./tests/fruit_test.xlsx";
    let mut range: Vec<Vec<String>> = vec![];

    let meta =
      read_excel(file_path, &mut range).expect("Failed to read \
                                                Excel file");

    assert_eq!(meta.1, 5);
    assert_eq!(meta.2, 5);

    assert_eq!(range[0][0], "APPLE");
    assert_eq!(range[1][1], "yellow");
    assert_eq!(range[2][2], "2024-08-29");
    assert_eq!(range[3][3], "1.13");
    assert_eq!(range[4][4], "Virginia");
  }
}
