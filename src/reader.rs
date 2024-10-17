use calamine::{open_workbook, Data, Reader, Xlsx};
use chrono::format::strftime::StrftimeItems;

pub fn read_excel(file_path: &str) -> Result<Vec<Vec<String>>, &str>
{
  let mut workbook: Xlsx<_> =
    open_workbook(file_path).expect("db2md: cannot open given xlsx \
                                     file");
  let sheet_name = workbook.sheet_names()[0].to_owned();
  if let Ok(range) = workbook.worksheet_range(&sheet_name) {
    let row_number = range.get_size().0;
    println!("db2md: found {row_number} rows in {sheet_name}");
    let mut sheet = vec![];
    for row in range.rows().into_iter() {
      sheet.push(parse_row(row));
    }
    return Ok(sheet);
  }
  Err("db2md: cannot read the sheet")
}

fn parse_row(row: &[Data]) -> Vec<String>
{
  // YYYY-MM-DD
  let fmt = StrftimeItems::new("%Y-%m-%d");
  let mut res: Vec<String> = vec![];
  for cell in row.iter() {
    match cell {
      Data::Int(x) => {
        let s = x.to_string();
        res.push(s);
      }
      Data::Float(x) => {
        let s = x.to_string();
        res.push(s);
      }
      Data::Bool(b) => {
        if *b {
          res.push(String::from("true"))
        } else {
          res.push(String::from("false"))
        }
      }
      Data::DateTime(t) => {
        if let Some(date) = t.as_datetime() {
          let s = date.format_with_items(fmt.clone()).to_string();
          res.push(s)
        } else {
          res.push(String::from(""))
        }
      }
      Data::String(s)
      | Data::DateTimeIso(s)
      | Data::DurationIso(s) => res.push(s.to_string()),
      _ => res.push(String::from("")),
    }
  }
  return res;
}
