use std::collections::{HashMap, HashSet};

pub fn generate_markdown(data_row: &[String],
                         field_map: &HashMap<String, usize>,
                         output: &mut String)
{
  if data_row.len() > field_map.len() {
    eprintln!("more data than specified fields");
    return ();
  } else if data_row.len() < field_map.len() {
    eprintln!("less data than specified fields");
    return ();
  }
  // sort keys in field_map based on its value (accending)
  let mut sorted_titles: Vec<&String> = field_map.keys().collect();
  sorted_titles.sort_by(|&a, &b| {
                 field_map.get(a)
                          .unwrap()
                          .cmp(field_map.get(b).unwrap())
               });
  let mut implemented_title: HashSet<&str> = HashSet::new();
  for &t in sorted_titles.iter() {
    // get content from each cell, "N/A" by default
    let default = String::from("N/A");
    let content = data_row.get(*field_map.get(t).unwrap())
                          .unwrap_or(&default);

    // prepare section title
    let sections = t.split('.').collect::<Vec<_>>();
    for (idx, &s) in sections.iter().enumerate() {
      if !implemented_title.contains(s) {
        implemented_title.insert(s);
        let new_title = format!("{} {}", "#".repeat(idx + 1), s);
        output.push_str(new_title.as_str());
        output.push('\n');
        output.push('\n');
      }
    }

    // append corresponding content
    output.push_str((*content).as_str());
    output.push('\n');
    output.push('\n');
  }
}
