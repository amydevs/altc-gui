use std::{borrow::BorrowMut, io::{Cursor, Seek}};

use crate::components::ask::{Ask, AskCard};

use yew::prelude::*;
use altc::util;

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
            Err(err) => return Err((err, reader.stream_position().unwrap())),
        };
        let converted = util::convert(parsed, to_version);
        let mut new_ask = fileinfo.clone();
        new_ask.contents = util::generate_ask(&converted).unwrap();
        new_ask.version = to_version;
        Ok::<Ask, (quick_xml::DeError, u64)>(new_ask)
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
                    for uploaded_file in uploaded_files {
                        let file_promise: js_sys::Promise = uploaded_file.text();
                        let file_text_js_value = wasm_bindgen_futures::JsFuture::from(file_promise)
                            .await
                            .unwrap();
                        let file_text = file_text_js_value.as_string().unwrap();
                        let mut new_files = (*files).clone();
                        new_files.push(Ask {
                            name: uploaded_file.name(),
                            version: util::get_live_version(&file_text).unwrap_or(util::LiveVersion::Live12),
                            contents: file_text,
                            to_version: Some(util::LiveVersion::Live12)
                        });
                        files.set(new_files);
                    }
                }
            });
        })
    };

    html! {
        <main>
            <div class="p-6">
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
            <div class="p-6">
                {
                    converted_ask_results.iter().map(|ask_result| {
                        if let Ok(ask) = ask_result {
                            html! {
                                <AskCard
                                   ask={ask.clone()}
                                />
                            }
                        }
                        else {
                            html!()
                        }
                   }).collect::<Vec<_>>()
                }
            </div>
            <div
                class="w-full h-96 border"
                ondrop={process_files}
                ondragover={Callback::from(|event: DragEvent| {
                    event.prevent_default();
                })}
                ondragenter={Callback::from(|event: DragEvent| {
                    event.prevent_default();
                })}
            >
                { "Drop Zone" }
            </div>
        </main>
    }
}
