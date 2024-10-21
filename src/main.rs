#![allow(non_snake_case)]
use db2md::gui::FilePrefixSetter;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main()
{
  // Init logger
  dioxus_logger::init(Level::INFO).expect("failed to init logger");
  info!("starting app");

  let cfg = dioxus::desktop::Config::new().with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string());
  LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element
{
  let file_path =
    use_signal(|| "please provide a path to data file".to_string());
  let yaml_path =
    use_signal(|| "please provide a path to yaml schema".to_string());
  let has_header = use_signal(|| true);
  let md_prefix: Signal<String> =
    use_signal(|| "ccms-doc".to_string());
  rsx! {
      link { rel: "stylesheet", href: "assets/main.css" }
      link { rel: "stylesheet", href: "assets/tailwind.css" }
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
      }
  }
}
