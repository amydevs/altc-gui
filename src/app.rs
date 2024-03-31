use std::{borrow::BorrowMut, io::{Cursor, Seek}};

use crate::components::ask::{Ask, AskCard};

use yew::prelude::*;
use altc::util;

pub struct DeErrorWrapper {
    pub name: String,
    pub cause: quick_xml::DeError,
    pub position: u64,
}

#[function_component(App)]
pub fn app() -> Html {
    let asks: UseStateHandle<Vec<crate::components::ask::Ask>> =
        use_state(std::vec::Vec::new);

    let converted_ask_results = asks.iter().map(|fileinfo| {
        let live_version = fileinfo.version;
        let to_version = fileinfo.to_version.unwrap();
        let cursor = Cursor::new(fileinfo.contents.as_bytes());
        let mut reader = std::io::BufReader::new(cursor);
        let parsed = match util::parse_ask_from_reader(reader.borrow_mut(), live_version) {
            Ok(parsed) => parsed,
            Err(err) => return Err(DeErrorWrapper {
                name: fileinfo.name.clone(),
                cause: err,
                position: reader.stream_position().unwrap()
            }),
        };
        let converted = util::convert(parsed, to_version);
        let mut new_ask = fileinfo.clone();
        new_ask.contents = util::generate_ask(&converted).unwrap();
        new_ask.version = to_version;
        new_ask.to_version = None;
        Ok::<Ask, DeErrorWrapper>(new_ask)
    }).collect::<Vec<_>>();

    let process_files = {
        let files = asks.clone();
        Callback::from(move |event: web_sys::DragEvent| {
            event.prevent_default();
            let files = files.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let files = files.clone();
                let uploaded_files = event.data_transfer().unwrap().files();
                if let Some(filelist) = uploaded_files {
                    let uploaded_files = js_sys::try_iter(&filelist)
                        .unwrap()
                        .unwrap()
                        .map(|v| web_sys::File::from(v.unwrap()))
                        .map(web_sys::File::from);
                    let mut new_files = (*files).clone();
                    for uploaded_file in uploaded_files {
                        let file_promise: js_sys::Promise = uploaded_file.text();
                        let file_text_js_value = wasm_bindgen_futures::JsFuture::from(file_promise)
                            .await
                            .unwrap();
                        let file_text = file_text_js_value.as_string().unwrap();
                        new_files.push(Ask {
                            name: uploaded_file.name(),
                            version: util::get_live_version(&file_text).unwrap_or(util::LiveVersion::Live12),
                            contents: file_text,
                            to_version: Some(util::LiveVersion::Live12)
                        });
                    }
                    files.set(new_files);
                }
            });
        })
    };

    html! {
        <main class="h-screen">
            <div class="flex p-6 gap-6 h-full">
                <div class="rounded-lg border bg-card text-card-foreground shadow-sm flex-1">
                    <div class="border-b p-6">
                        { "Drag .ask files here" }
                    </div>
                    <div
                        class="p-6 h-full space-y-3"
                        ondrop={process_files}
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
                                            let files = asks.clone();
                                            {
                                                Callback::from(move |value| {
                                                    let mut new_files = (*files).clone();
                                                    new_files[i] = value;
                                                    files.set(new_files);
                                                })
                                            }
                                        }
                                    />
                                }
                            }).collect::<Vec<_>>()
                        }
                    </div>
                </div>
                <div class="rounded-lg border bg-card text-card-foreground shadow-sm flex-1">
                    <div class="border-b p-6">
                            { "Output .ask files" }
                    </div>
                    <div class="p-6 space-y-3">
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
                                            <div class="rounded-lg border bg-card text-card-foreground shadow-sm p-3 flex items-center justify-between">
                                                <div>
                                                    { err.name.clone() }
                                                </div>
                                                <div>
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
