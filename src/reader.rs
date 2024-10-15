use calamine::{open_workbook, Reader, Xlsx};
use std::error::Error;

pub fn read_excel(file_path: &str)
                  -> Result<Vec<Vec<String>>, Box<dyn Error>>
{
  let mut workbook: Xlsx<_> = open_workbook(file_path)?;
  let sheet_name = workbook.sheet_names()[0].to_owned();
  let range = workbook.worksheet_range(&sheet_name)
                      .ok_or("Cannot find sheet")??;

  let rows =
    range.rows()
         .map(|row| row.iter().map(|cell| cell.to_string()).collect())
         .collect();

  Ok(rows)
}
