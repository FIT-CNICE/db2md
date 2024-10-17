use calamine::{open_workbook, Data, Range, Reader, Xlsx};

pub fn read_excel(file_path: &str) -> Result<Range<Data>, &str>
{
  let mut workbook: Xlsx<_> =
    open_workbook(file_path).expect("db2md: cannot open given xlsx \
                                     file");
  let sheet_name = workbook.sheet_names()[0].to_owned();
  if let Ok(range) = workbook.worksheet_range(&sheet_name) {
    let row_number = range.get_size().0;
    println!("db2md: found {row_number} rows in {sheet_name}");
    return Ok(range);
  }
  Err("db2md: cannot read the sheet")
}
