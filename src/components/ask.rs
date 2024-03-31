use std::mem::transmute;

use altc::util::LiveVersion;
use gloo::*;
use js_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlSelectElement, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::components::live_version_select_options::LiveVersionSelectOptions;

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
    pub class: Classes,
    pub ask: Ask,
    #[prop_or_default]
    pub onedit: Callback<Ask>,
}

#[function_component(AskCard)]
pub fn file(props: &Props) -> Html {
    let active = use_state(|| false);
    html! {
        <div class={ classes!("rounded-lg", "border", "bg-card", "text-card-foreground", "shadow-sm", props.class.clone()) }>
            // Heading
            <div class="p-3 flex items-center justify-between border-b">
                { &props.ask.name }
                <button
                    onclick={{
                        let active = active.clone();
                        Callback::from(move |_| {
                            active.set(!*active);
                        })
                    }}
                >
                    <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M4.18179 6.18181C4.35753 6.00608 4.64245 6.00608 4.81819 6.18181L7.49999 8.86362L10.1818 6.18181C10.3575 6.00608 10.6424 6.00608 10.8182 6.18181C10.9939 6.35755 10.9939 6.64247 10.8182 6.81821L7.81819 9.81821C7.73379 9.9026 7.61934 9.95001 7.49999 9.95001C7.38064 9.95001 7.26618 9.9026 7.18179 9.81821L4.18179 6.81821C4.00605 6.64247 4.00605 6.35755 4.18179 6.18181Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path></svg>
                </button>
            </div>
            <div class="p-3">
                {
                    if let Some(to_version) = props.ask.to_version {
                        html! {
                            <>
                                { "Convert from " }
                                <select class="p-1" onchange={{
                                    let onedit = props.onedit.clone();
                                    let ask = props.ask.clone();
                                    Callback::from(move |event: Event| {
                                        let area: HtmlSelectElement = event.target().unwrap().dyn_into().unwrap();
                                        let version = LiveVersion::from_u8(area.value().parse::<u8>().unwrap()).unwrap();
                                        let mut new_ask = ask.clone();
                                        new_ask.version = version;
                                        onedit.emit(new_ask);
                                    })
                                }}>
                                    <LiveVersionSelectOptions value={props.ask.version} />
                                </select>
                                { " to " }
                                <select class="p-1" onchange={{
                                    let onedit = props.onedit.clone();
                                    let ask = props.ask.clone();
                                    Callback::from(move |event: Event| {
                                        let area: HtmlSelectElement = event.target().unwrap().dyn_into().unwrap();
                                        let version = LiveVersion::from_u8(area.value().parse::<u8>().unwrap()).unwrap();
                                        let mut new_ask = ask.clone();
                                        new_ask.to_version = Some(version);
                                        onedit.emit(new_ask);
                                    })
                                }}>
                                    <LiveVersionSelectOptions value={to_version} />
                                </select>
                            </>
                        }
                    }
                    else {
                        html! {
                            { format!("Converted to Live {}", props.ask.version as u8) }
                        }
                    }
                    
                }
                // Popout
                <div class={ classes!(if *active { "" } else { "hidden" }) }>
                    <textarea
                        class="resize-y w-full h-64"
                        value={ props.ask.contents.clone() }
                        oninput={{
                            let onedit = props.onedit.clone();
                            let ask = props.ask.clone();
                            Callback::from(move |event: InputEvent| {
                                let area: HtmlTextAreaElement = event.target().unwrap().dyn_into().unwrap();
                                let mut new_ask = ask.clone();
                                new_ask.contents = area.value();
                                onedit.emit(new_ask);
                            })
                        }}
                    />
                </div>
            </div>
            
        </div>
    }
}
