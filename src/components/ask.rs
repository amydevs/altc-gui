use altc::util::LiveVersion;
use js_sys::wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Ask {
    pub name: String,
    pub contents: String,
    pub version: LiveVersion,
    pub to_version: Option<LiveVersion>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub classes: Classes,
    pub ask: Ask,
    #[prop_or_default]
    pub onedit: Callback<Ask>,
}

#[function_component(AskCard)]
pub fn file(props: &Props) -> Html {
    let active = use_state(|| false);
    let onedit = props.onedit.clone();
    let ask = props.ask.clone();
    html! {
        <div class={ classes!("rounded-lg", "border", "bg-card", "text-card-foreground", "shadow-sm", props.classes.clone()) }>
            // Heading
            <div class="class p-3 flex items-center justify-between">
                { &props.ask.name }
                <button onclick={{
                    let active = active.clone();
                    Callback::from(move |_| {
                        active.set(!*active);
                    })
                }}>
                    { "More" }
                </button>
            </div>
            // Popout
            <div class={ classes!(if *active { "" } else { "hidden" }, "p-3") }>
                <textarea
                    class="resize-y w-full h-64"
                    value={ props.ask.contents.clone() }
                    oninput={Callback::from(move |event: InputEvent| {
                        let area: HtmlTextAreaElement = event.target().unwrap().dyn_into().unwrap();
                        let mut new_ask = ask.clone();
                        new_ask.contents = area.value();
                        onedit.emit(new_ask);
                    })}
                />
            </div>
        </div>
    }
}
