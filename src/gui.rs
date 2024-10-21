use dioxus::prelude::*;
use dioxus::signals::*;

#[component]
pub fn FilePrefixSetter(md_prefix: Signal<String>) -> Element
{
  let mut input_value = use_signal(|| md_prefix());
  rsx! {
      div {
          span { "Your File Prefix: " }
          input {
              class: "bg-gray-700 text-white border border-gray-700 rounded px-3 py-2 mx-2",
              value: "{input_value}",
              oninput: move |evt| {input_value.set(evt.value())},
          }
          button {
            class: "bg-blue-600 hover:bg-blue-700 text-white border border-blue-700 font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline",
            onclick: move |event| {md_prefix.set(input_value())}, "Set"
  }
      }
  }
}
