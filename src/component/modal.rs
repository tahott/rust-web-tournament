use gloo::{utils::document, console::console};
use wasm_bindgen::{JsValue, JsCast};
use web_sys::{console, HtmlSelectElement, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
  pub state: bool
}

#[function_component(TournamentModal)]
pub fn Modal(props: &ModalProps) -> Html {
  let onclick = {
    Callback::from(move |_| {
      let val = parse_element::<HtmlSelectElement>("tournament_type").value();
      let participants = parse_element::<HtmlInputElement>("participants").value_as_number();
      let title = parse_element::<HtmlInputElement>("title").value();
      console::log_3(&JsValue::from_str(&val), &JsValue::from_f64(participants), &JsValue::from_str(&title));
    })
  };

  html!(
    <div class={if props.state {"max-w-lg w-full absolute mx-auto p-4 text-white rounded-lg shadow-2xl bg-indigo-500 shadow-indigo-500/50"} else {"hidden"}}>
      <div class="grid grid-cols-2 gap-2 mb-6">
        <select id="tournament_type" class="bg-indigo-400">
          <option value="토너먼트" selected={true}>{"토너먼트"}</option>
          <option value="스위스 리그" disabled={true}>{"스위스 리그"}</option>
        </select>
        <input id="participants" class="bg-indigo-400 rounded" type="number" min=2 max=64 />
      </div>
      <div>
        <input id="title" class="rounded w-full pl-2 mb-6 bg-indigo-400" type="text" placeholder="title" />
      </div>
      <div>
        <button type="button" class="rounded bg-indigo-200 w-full" {onclick}>{"생성"}</button>
      </div>
    </div>
  )
}

fn parse_element<T: wasm_bindgen::JsCast>(id: &str) -> T {
  document().get_element_by_id(id).unwrap().dyn_into::<T>().unwrap()
}