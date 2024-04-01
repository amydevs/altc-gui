use std::{borrow::BorrowMut, io::{Cursor, Seek}};

use crate::components::ask::{Ask, AskCard};
use crate::components::live_version_select_options::LiveVersionSelectOptions;

use js_sys::wasm_bindgen::JsCast;
use web_sys::{FileList, HtmlAnchorElement, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use altc::util::{self, LiveVersion};

pub struct DeErrorWrapper {
    pub name: String,
    pub cause: quick_xml::DeError,
    pub line_number: u64,
}

#[function_component(App)]
pub fn app() -> Html {
    let default_version_to = use_state(|| LiveVersion::Live12);
    let asks: UseStateHandle<Vec<crate::components::ask::Ask>> =
        use_state(std::vec::Vec::new);
    let downloader_ref = use_node_ref();

    let converted_ask_results = asks.iter().map(|fileinfo| {
        let live_version = fileinfo.version;
        let to_version = fileinfo.to_version.unwrap();
        let cursor = Cursor::new(fileinfo.contents.as_bytes());
        let mut reader = std::io::BufReader::new(cursor);
        
        let parsed = match util::parse_ask_from_reader(reader.borrow_mut(), live_version) {
            Ok(parsed) => parsed,
            Err(err) => {
                let line_number = fileinfo.contents[..reader.stream_position().unwrap() as usize]
                    .chars()
                    .filter(|x| *x == '\n')
                    .count();
                return Err(DeErrorWrapper {
                    name: fileinfo.name.clone(),
                    cause: err,
                    line_number: line_number as u64,
                })
            },
        };
        let converted = util::convert(parsed, to_version);
        let mut new_ask = fileinfo.clone();
        new_ask.contents = util::generate_ask(&converted).unwrap();
        new_ask.version = to_version;
        new_ask.to_version = None;
        Ok::<Ask, DeErrorWrapper>(new_ask)
    }).collect::<Vec<_>>();

    let process_uploaded_files = {
        let asks = asks.clone();
        let default_version_to = default_version_to.clone();
        Callback::from(move |filelist: FileList| {
            let asks = asks.clone();
            let default_version_to = default_version_to.clone();
            wasm_bindgen_futures::spawn_local(async move {
            let asks = asks.clone();
                let uploaded_files = js_sys::try_iter(&filelist)
                    .unwrap()
                    .unwrap()
                    .map(|v| web_sys::File::from(v.unwrap()))
                    .map(web_sys::File::from);
                let mut new_asks = (*asks).clone();
                for uploaded_file in uploaded_files {
                    let file_promise: js_sys::Promise = uploaded_file.text();
                    let file_text_js_value = wasm_bindgen_futures::JsFuture::from(file_promise)
                        .await
                        .unwrap();
                    let file_text = file_text_js_value.as_string().unwrap();
                    let default_version_to = default_version_to.clone();
                    new_asks.push(Ask {
                        name: uploaded_file.name(),
                        version: util::get_live_version(&file_text).unwrap_or(util::LiveVersion::Live12),
                        contents: file_text,
                        to_version: Some(*default_version_to)
                    });
                }
                asks.set(new_asks);
            });
        })
    };

    html! {
        <main class="h-screen bg-background">
            <div class="p-6 pb-0 flex min-h-[33.333%]">
                <div class="rounded-lg border bg-card text-card-foreground shadow-sm p-6 space-y-3 flex-1">
                    <div>
                        <h1 class="font-bold">
                            { "Ableton Live Theme Converter GUI" }
                        </h1>
                        <p>
                            { "AltC is a CLI/web tool built in Rust that converts any Live >= 10 theme to be compatible with any other Live version >= 10." }
                        </p>
                        <p>
                            { "Please send me emails at " }
                            <a class="underline" href="mailto:ayanamydev@gmail.com">{ "ayanamydev@gmail.com" }</a>
                            { " if you have any problems or suggestions for improvements (screenshots would be appreciated)." }
                        </p>
                    </div>
                    <div>
                        <h2 class="font-bold">{ "Links" }</h2>
                        <ul class="list-disc pl-3">
                            <li><a class="underline" href="https://github.com/amydevs/altc-gui">{ "Source code" }</a></li>
                            <li><a class="underline" href="https://github.com/amydevs/AbletonLiveThemeConverter">{ "CLI source code" }</a></li>
                            <li><a class="underline" href="buymeacoffee.com/amydev">{ "Buy me a cofee" }</a></li>
                        </ul>
                    </div>
                    <div>
                        <h2 class="font-bold">{ "Options" }</h2> 
                        { "By default convert to " }
                        <select class="p-1 bg-background border" onchange={{
                            let default_version_to = default_version_to.clone();
                            Callback::from(move |event: Event| {
                                let area: HtmlSelectElement = event.target().unwrap().dyn_into().unwrap();
                                let version = LiveVersion::from_u8(area.value().parse::<u8>().unwrap()).unwrap();
                                default_version_to.set(version);
                            })
                        }}>
                            <LiveVersionSelectOptions />
                        </select>                 
                    </div>
                </div>
            </div>
            <div class="flex sm:flex-row flex-col p-6 gap-6 h-2/3">
                <div class="flex flex-col rounded-lg border bg-card text-card-foreground shadow-sm flex-1">
                    <div class="border-b p-6 flex justify-between">
                        { "Drag .ask files below here" }
                        <input
                            id="uploader"
                            type="file"
                            multiple={true}
                            class="hidden" 
                            onchange={{
                                let process_uploaded_files = process_uploaded_files.clone();
                                Callback::from(move |event: Event| {
                                    let input: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
                                    if let Some(uploaded_files) = input.files() {
                                        process_uploaded_files.emit(uploaded_files);
                                    }
                                })
                            }}
                        />
                        <label title="Upload .ask files" tabindex="0" class="cursor-pointer" for="uploader">
                            <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M7.81825 1.18188C7.64251 1.00615 7.35759 1.00615 7.18185 1.18188L4.18185 4.18188C4.00611 4.35762 4.00611 4.64254 4.18185 4.81828C4.35759 4.99401 4.64251 4.99401 4.81825 4.81828L7.05005 2.58648V9.49996C7.05005 9.74849 7.25152 9.94996 7.50005 9.94996C7.74858 9.94996 7.95005 9.74849 7.95005 9.49996V2.58648L10.1819 4.81828C10.3576 4.99401 10.6425 4.99401 10.8182 4.81828C10.994 4.64254 10.994 4.35762 10.8182 4.18188L7.81825 1.18188ZM2.5 9.99997C2.77614 9.99997 3 10.2238 3 10.5V12C3 12.5538 3.44565 13 3.99635 13H11.0012C11.5529 13 12 12.5528 12 12V10.5C12 10.2238 12.2239 9.99997 12.5 9.99997C12.7761 9.99997 13 10.2238 13 10.5V12C13 13.104 12.1062 14 11.0012 14H3.99635C2.89019 14 2 13.103 2 12V10.5C2 10.2238 2.22386 9.99997 2.5 9.99997Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path></svg>
                        </label>
                    </div>
                    <div
                        class="p-6 flex-1 space-y-3 overflow-y-auto relative"
                        ondrop={Callback::from(move |event: DragEvent| {
                            event.prevent_default();
                            if let Some(uploaded_files) = event.data_transfer().unwrap().files() {
                                process_uploaded_files.emit(uploaded_files);
                            }
                        })}
                        ondragover={Callback::from(|event: DragEvent| {
                            event.prevent_default();
                        })}
                        ondragenter={Callback::from(|event: DragEvent| {
                            event.prevent_default();
                        })}
                    >
                        {
                            asks.iter().enumerate().map(|(i, ask)| {
                                html! {
                                    <AskCard
                                        ask={ask.clone()}
                                        onedit={
                                            let asks = asks.clone();
                                            {
                                                Callback::from(move |value| {
                                                    let mut new_asks = (*asks).clone();
                                                    new_asks[i] = value;
                                                    asks.set(new_asks);
                                                })
                                            }
                                        }
                                        ondelete={
                                            let asks = asks.clone();
                                            Callback::from(move |_| {
                                                let mut new_asks = (*asks).clone();
                                                new_asks.remove(i);
                                                asks.set(new_asks);
                                            })
                                        }
                                    />
                                }
                            }).collect::<Vec<_>>()
                        }
                        <div class={classes!("absolute", "top-1/2", "left-1/2", "-translate-x-1/2", "-translate-y-1/2", "z-0", "text-primary", if asks.len() == 0 {""} else {"hidden"})}>
                            <svg class="h-32 w-32" width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M7.81825 1.18188C7.64251 1.00615 7.35759 1.00615 7.18185 1.18188L4.18185 4.18188C4.00611 4.35762 4.00611 4.64254 4.18185 4.81828C4.35759 4.99401 4.64251 4.99401 4.81825 4.81828L7.05005 2.58648V9.49996C7.05005 9.74849 7.25152 9.94996 7.50005 9.94996C7.74858 9.94996 7.95005 9.74849 7.95005 9.49996V2.58648L10.1819 4.81828C10.3576 4.99401 10.6425 4.99401 10.8182 4.81828C10.994 4.64254 10.994 4.35762 10.8182 4.18188L7.81825 1.18188ZM2.5 9.99997C2.77614 9.99997 3 10.2238 3 10.5V12C3 12.5538 3.44565 13 3.99635 13H11.0012C11.5529 13 12 12.5528 12 12V10.5C12 10.2238 12.2239 9.99997 12.5 9.99997C12.7761 9.99997 13 10.2238 13 10.5V12C13 13.104 12.1062 14 11.0012 14H3.99635C2.89019 14 2 13.103 2 12V10.5C2 10.2238 2.22386 9.99997 2.5 9.99997Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path></svg>
                        </div>
                    </div>
                </div>
                <div class="flex flex-col rounded-lg border bg-card text-card-foreground shadow-sm flex-1">
                    <div class="border-b p-6 flex justify-between">
                        { "Output .ask files" }
                        <a
                            ref={downloader_ref.clone()}
                            class="hidden"
                        />
                        <button
                            title="Download.ask files"
                            onclick={{
                                let asks = asks.clone();
                                Callback::from(move |_| {
                                    let asks = asks.clone();
                                    for ask in asks.iter() {
                                        let contents_jsvalue = wasm_bindgen::JsValue::from_str(&ask.contents);
                                        let contents_jsvalue_array = js_sys::Array::from_iter(std::iter::once(contents_jsvalue));
                                        let blob = web_sys::Blob::new_with_str_sequence(&contents_jsvalue_array).unwrap();
                                        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                                        let anchor = downloader_ref.cast::<HtmlAnchorElement>().unwrap();
                                        anchor.set_download(&ask.name);
                                        anchor.set_href(&url);
                                        anchor.click();
                                        web_sys::Url::revoke_object_url(&url).unwrap();
                                    }
                                })
                            }}
                        >
                            <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M7.50005 1.04999C7.74858 1.04999 7.95005 1.25146 7.95005 1.49999V8.41359L10.1819 6.18179C10.3576 6.00605 10.6425 6.00605 10.8182 6.18179C10.994 6.35753 10.994 6.64245 10.8182 6.81819L7.81825 9.81819C7.64251 9.99392 7.35759 9.99392 7.18185 9.81819L4.18185 6.81819C4.00611 6.64245 4.00611 6.35753 4.18185 6.18179C4.35759 6.00605 4.64251 6.00605 4.81825 6.18179L7.05005 8.41359V1.49999C7.05005 1.25146 7.25152 1.04999 7.50005 1.04999ZM2.5 10C2.77614 10 3 10.2239 3 10.5V12C3 12.5539 3.44565 13 3.99635 13H11.0012C11.5529 13 12 12.5528 12 12V10.5C12 10.2239 12.2239 10 12.5 10C12.7761 10 13 10.2239 13 10.5V12C13 13.1041 12.1062 14 11.0012 14H3.99635C2.89019 14 2 13.103 2 12V10.5C2 10.2239 2.22386 10 2.5 10Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path></svg>
                        </button>
                    </div>
                    <div class="p-6 space-y-3 flex-1 overflow-y-auto">
                        {
                            converted_ask_results.iter().map(|ask_result| {
                                match ask_result {
                                    Ok(ask) => {
                                        html! {
                                            <AskCard
                                                ask={ask.clone()}
                                            />
                                        }
                                    },
                                    Err(err) => {
                                        html!(
                                            <div class="rounded-lg border bg-card text-card-foreground shadow-sm">
                                                <div class="p-3 flex items-center justify-between border-b">
                                                    <div>
                                                        { err.name.clone() }
                                                    </div>
                                                    <div>
                                                        { format!("Error on line {}", err.line_number) }
                                                    </div>
                                                </div>
                                                <div class="p-3">
                                                    { err.cause.clone() }
                                                </div>
                                            </div>
                                        )
                                    },
                                }
                            }).collect::<Vec<_>>()
                        }
                    </div>
                </div>
            </div>
        </main>
    }
}
