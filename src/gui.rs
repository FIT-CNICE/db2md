#![allow(non_snake_case)]
use crate::reader::read_excel;
use dioxus::prelude::*;

#[component]
pub fn FilePrefixSetter(md_prefix: Signal<String>) -> Element
{
  let mut input_value = use_signal(|| md_prefix());
  rsx! {
      div {
          span { class: "px-2", "Your File Prefix: " }
          input {
              class: "bg-gray-700 text-white border border-gray-700 rounded px-3 py-2 mx-2",
              value: "{input_value}",
              oninput: move |evt| {input_value.set(evt.value())},
          }
          button {
            class: "bg-blue-600 hover:bg-blue-700 text-white border border-blue-700 font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline",
            onclick: move |_evt| {md_prefix.set(input_value())}, "Set"
  }
      }
  }
}

#[component]
pub fn FileLoader(file_path: Signal<String>,
                  table_meta: Signal<(String, usize, usize)>,
                  data_mtx: Signal<Vec<Vec<String>>>)
                  -> Element
{
  let mut err_msg = use_signal(|| String::new());
  let mut meta_msg = use_signal(|| String::new());

  rsx! {
    div {
        span { class:"px-2 mr-5", "Select Data File:" }
        input {
          r#type:"file",
          value: "{file_path}",
          accept: ".xlsx",
          class: "bg-gray-700 text-white border border-gray-700 rounded px-3 py-2 mx-2",
          multiple: false,
          onchange: move |evt| {
            if let Some(file_engine) = &evt.files() {
              let selected = file_engine.files();
              if let Some(first_file) = selected.first() {
                  file_path.set(first_file.to_string());
              }
            }
          }
        }

        button {
            class: "bg-blue-600 hover:bg-blue-700 text-white border border-blue-700 font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline",
            onclick: move |_evt| {
              let file = file_path();
              let mut raw_data = vec![];
              let res = read_excel(&file, &mut raw_data);
              match res {
                Ok(meta)=>{
                  meta_msg.set(format!("{} rows of {} strings from {}", meta.1, meta.2, meta.0));
                  err_msg.set(String::new());
                  data_mtx.set(raw_data);
                  table_meta.set(meta);
                }
                Err(msg) => {
                  err_msg.set(msg);
                  meta_msg.set(String::new());
                  data_mtx.clear();
                  table_meta.set((String::from(""),0,0))
                }
              }
            }, "Load"
        }
    }
  }
}

#[component]
pub fn HeaderChecker(has_header: Signal<bool>) -> Element
{
  rsx! {
      div {
         span { class: "px-2 mr-5", "First-row Header?" }
         input {
           r#type:"radio",
           class:"peer px-2 h-5 w-5 cursor-pointer transition-all appearance-none rounded shadow hover:shadow-md border border-slate-300 checked:bg-blue-600 checked:border-blue-600",
           id:"has-header",
           name:"options",
           value:"true",
           onclick: move |_evt| {has_header.set(true)}
         }
         span { class: "px-2 mr-5", "Yes" }
         input {
           r#type:"radio",
           class:"peer px-2 h-5 w-5 cursor-pointer transition-all appearance-none rounded shadow hover:shadow-md border border-slate-300 checked:bg-blue-600 checked:border-blue-600",
           id:"no-header",
           name:"options",
           value:"false",
           onclick: move |_evt| {has_header.set(false)}
         }
         span { class: "px-2 mr-5", "No" }
      }
  }
}
