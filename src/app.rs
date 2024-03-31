use yew::prelude::*;
use gloo::console::log;
use crate::components::filelist::file::File;

#[function_component(App)]
pub fn app() -> Html {
    let files: UseStateHandle<Vec<crate::components::filelist::file::FileInfo>> = use_state(|| vec![]);
    
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
                        let file_text_js_value = wasm_bindgen_futures::JsFuture::from(file_promise).await.unwrap();
                        let mut new_files = (*files).clone();
                        new_files.push(crate::components::filelist::file::FileInfo {
                            name: uploaded_file.name(),
                            contents: file_text_js_value.as_string().unwrap()
                        });
                        
                        files.set(new_files);
                    }
                }
            });

        })
    };

    html! {
        <main>
            {
                files.iter().enumerate().map(|(i, fileinfo)| {
                    html! {
                        <File file={fileinfo.clone()} />
                    }
                }).collect::<Vec<_>>()
            }
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
