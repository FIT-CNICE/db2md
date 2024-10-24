use std::collections::{HashMap, HashSet};

pub fn generate_markdown(data_row: &[String],
                         field_map: &HashMap<String, usize>,
                         output: &mut String)
{
  // sort keys in field_map based on its value (accending)
  let mut sorted_titles: Vec<&String> = field_map.keys().collect();
  sorted_titles.sort_by(|&a, &b| {
                 field_map.get(a)
                          .unwrap()
                          .cmp(field_map.get(b).unwrap())
               });
  let mut implemented_title: HashSet<String> = HashSet::new();
  let mut current_path = Vec::new();
  for &t in sorted_titles.iter() {
    // get content from each cell, "N/A" by default
    let default = String::from("N/A");
    let content = data_row.get(*field_map.get(t).unwrap())
                          .unwrap_or(&default);

    // prepare section title
    let sections = t.split('.').collect::<Vec<_>>();
    for (idx, &s) in sections.iter().enumerate() {
      current_path.truncate(idx);
      current_path.push(s);
      let path_str = current_path.join(".");
      if !implemented_title.contains(&path_str) {
        // return owned values to solve lifetime issue
        implemented_title.insert(path_str.to_owned());
        let new_title = format!("{} {}", "#".repeat(idx + 1), s);
        output.push_str(&new_title);
        output.push_str("\n\n");
      }
    }

    // append corresponding content
    output.push_str((*content).as_str());
    output.push('\n');
    output.push('\n');
  }
}
