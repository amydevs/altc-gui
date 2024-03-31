use altc::util::LiveVersion;
use gloo::*;
use js_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, HtmlSelectElement, HtmlTextAreaElement};
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
    pub ondelete: Option<Callback<()>>,
}

#[function_component(AskCard)]
pub fn file(props: &Props) -> Html {
    let active = use_state(|| false);
    let downloader_ref = use_node_ref();
    html! {
        <div class={ classes!("rounded-lg", "border", "bg-card", "text-card-foreground", "shadow-sm", props.class.clone()) }>
            // Heading
            <div class="p-3 flex items-center justify-between border-b">
                { &props.ask.name }
                <div class="space-x-3">
                    {
                        if props.ask.to_version.is_none() {
                            html! {
                                <>
                                    <a
                                        ref={downloader_ref.clone()}
                                        class="hidden"
                                    />
                                    <button
                                        onclick={{
                                            let ask = props.ask.clone();
                                            Callback::from(move |_| {
                                                let contents_jsvalue = wasm_bindgen::JsValue::from_str(&ask.contents);
                                                let contents_jsvalue_array = js_sys::Array::from_iter(std::iter::once(contents_jsvalue));
                                                let blob = web_sys::Blob::new_with_str_sequence(&contents_jsvalue_array).unwrap();
                                                let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                                                let anchor = downloader_ref.cast::<HtmlAnchorElement>().unwrap();
                                                anchor.set_download(&ask.name);
                                                anchor.set_href(&url);
                                                let anchor = downloader_ref.cast::<HtmlAnchorElement>().unwrap();
                                                anchor.click();
                                                web_sys::Url::revoke_object_url(&url).unwrap();
                                            })
                                        }}
                                    >
                                        <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M7.50005 1.04999C7.74858 1.04999 7.95005 1.25146 7.95005 1.49999V8.41359L10.1819 6.18179C10.3576 6.00605 10.6425 6.00605 10.8182 6.18179C10.994 6.35753 10.994 6.64245 10.8182 6.81819L7.81825 9.81819C7.64251 9.99392 7.35759 9.99392 7.18185 9.81819L4.18185 6.81819C4.00611 6.64245 4.00611 6.35753 4.18185 6.18179C4.35759 6.00605 4.64251 6.00605 4.81825 6.18179L7.05005 8.41359V1.49999C7.05005 1.25146 7.25152 1.04999 7.50005 1.04999ZM2.5 10C2.77614 10 3 10.2239 3 10.5V12C3 12.5539 3.44565 13 3.99635 13H11.0012C11.5529 13 12 12.5528 12 12V10.5C12 10.2239 12.2239 10 12.5 10C12.7761 10 13 10.2239 13 10.5V12C13 13.1041 12.1062 14 11.0012 14H3.99635C2.89019 14 2 13.103 2 12V10.5C2 10.2239 2.22386 10 2.5 10Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path></svg>
                                    </button>
                                </>
                            }
                        }
                        else {
                            html! {
                            }
                        }
                    }
                    {
                        if let Some(ondelete) = &props.ondelete {
                            html! {
                                <button
                                    onclick={{
                                        let ondelete = ondelete.clone();
                                        Callback::from(move |_| {
                                            ondelete.emit(());
                                        })
                                    }}
                                >
                                    <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M5.5 1C5.22386 1 5 1.22386 5 1.5C5 1.77614 5.22386 2 5.5 2H9.5C9.77614 2 10 1.77614 10 1.5C10 1.22386 9.77614 1 9.5 1H5.5ZM3 3.5C3 3.22386 3.22386 3 3.5 3H5H10H11.5C11.7761 3 12 3.22386 12 3.5C12 3.77614 11.7761 4 11.5 4H11V12C11 12.5523 10.5523 13 10 13H5C4.44772 13 4 12.5523 4 12V4L3.5 4C3.22386 4 3 3.77614 3 3.5ZM5 4H10V12H5V4Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path></svg>
                                </button>
                            }
                        }
                        else {
                            html! {
                            }
                        }
                    }
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
            </div>
            <div class="p-3">
                {
                    if let Some(to_version) = props.ask.to_version {
                        html! {
                            <>
                                { "Convert from " }
                                <select class="p-1 bg-background border" onchange={{
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
                                <select class="p-1 bg-background border" onchange={{
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
                <div class={ classes!(if *active { "" } else { "hidden" }, "pt-3") }>
                    <span>{ "Contents:" }</span>
                    <textarea
                        class="resize-y w-full h-64 rounded-lg border bg-card text-card-foreground shadow-sm"
                        value={props.ask.contents.clone()}
                        disabled={props.ask.to_version.is_none()}
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
