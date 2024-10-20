#![allow(non_snake_case)]

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
  rsx! {
      link { rel: "stylesheet", href: "assets/main.css" }
      img { src: "assets/header.svg", id: "header" }
      div { id: "links",
          a { href: "https://dioxuslabs.com/learn/0.5/", "📚 Learn Dioxus" }
          a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
          a { href: "https://github.com/DioxusLabs/dioxus-std", "⚙️ Dioxus Standard Library" }
          a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
      }
  }
}
