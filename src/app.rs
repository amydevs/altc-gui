use std::io::Cursor;

use crate::components::filelist::file::File;

use gloo::{dialogs::alert, *};
use yew::prelude::*;
use altc::util;

#[function_component(App)]
pub fn app() -> Html {
    let files: UseStateHandle<Vec<crate::components::filelist::file::Ask>> =
        use_state(std::vec::Vec::new);

    let converted_files = files.iter().map(|fileinfo| {
        let live_version = fileinfo.version;
        let to_version = fileinfo.to_version.unwrap();
        let cursor = Cursor::new(fileinfo.contents.as_bytes());
        let reader = std::io::BufReader::new(cursor);
        let parsed = util::parse_ask_from_reader(reader, live_version).unwrap();
        let converted = util::convert(parsed, to_version);
        let mut new_fileinfo = fileinfo.clone();
        new_fileinfo.contents = util::generate_ask(&converted).unwrap();
        new_fileinfo.version = to_version;
        new_fileinfo
    }).collect::<Vec<_>>();

    let process_files = {
        let files = files.clone();
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
                        if let Some(live_version) = util::get_live_version(&file_text) {
                            new_files.push(crate::components::filelist::file::Ask {
                                name: uploaded_file.name(),
                                contents: file_text,
                                version: live_version,
                                to_version: Some(util::LiveVersion::Live12)
                            });
                            files.set(new_files);
                        }
                        else {
                            alert(&format!("Could not detect Live version of \"{}\"", uploaded_file.name()))
                        }
                        
                    }
                }
            });
        })
    };

    html! {
        <main>
            <div class="p-6">
                {
                    files.iter().enumerate().map(|(i, fileinfo)| {
                        html! {
                            <File
                                ask={fileinfo.clone()}
                                onedit={
                                    let files = files.clone();
                                    {
                                        Callback::from(move |value| {
                                            let mut new_files = (*files).clone();
                                            new_files[i].contents = value;
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
                    converted_files.iter().map(|fileinfo| {
                        html! {
                            <File
                                ask={fileinfo.clone()}
                            />
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
