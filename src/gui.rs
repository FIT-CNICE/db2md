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
            onclick: move |event| {md_prefix.set(input_value())}, "Set"
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
