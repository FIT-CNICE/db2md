#![allow(non_snake_case)]
use db2md::gui::{FileLoader, FilePrefixSetter, HeaderChecker};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main()
{
  // Init logger
  dioxus_logger::init(Level::INFO).expect("failed to init logger");
  info!("starting app");

  let cfg = dioxus::desktop::Config::new().with_custom_head(r#"<link rel="stylesheet" href="assets/tailwind.css">"#.to_string());
  LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element
{
  // file-relevant signals
  let file_path = use_signal(|| "No File Selected".to_string());
  let table_meta = use_signal(|| (String::from(""), 0, 0));
  let data_mtx: Signal<Vec<Vec<String>>> = use_signal(|| Vec::new());
  let has_header = use_signal(|| true);

  // shcema-relevant signals
  let _yaml_path = use_signal(|| "No Yaml Selected".to_string());

  // md-relevant signals
  let md_prefix: Signal<String> =
    use_signal(|| "ccms-doc".to_string());
  rsx! {
      link { rel: "stylesheet", href: "assets/main.css" }
      img { src: "assets/header.svg", id: "header" }
      div { id: "content",
          FilePrefixSetter{ md_prefix }
          div {
              span{ class: "px-2", "MD filename prefix is set to: "}
              span{
                class: "px-2 text-red-500 font-bold",
                "{md_prefix}"
              }
          }
          HeaderChecker{ has_header }
          FileLoader { file_path, table_meta, data_mtx }
      }
  }
}
