use yew::prelude::*;
use gloo::console::log;
use std::sync::atomic::{AtomicU16, Ordering};
use std::{collections::HashMap, rc::Rc};

#[function_component(App)]
pub fn app() -> Html {
    let filereaders = use_state(|| HashMap::<u16, Rc<gloo::file::callbacks::FileReader>>::new());
    let filereader_ids = use_state(|| AtomicU16::new(0));

    let process_files = {
        let filereaders = filereaders.clone();
        let filereader_ids = filereader_ids.clone();
        Callback::from(move |event: web_sys::DragEvent| {
            event.prevent_default();
            let filereaders_for_remove = filereaders.clone();
            let files = event.data_transfer().unwrap().files();
            if let Some(filelist) = files {
                let files = js_sys::try_iter(&filelist)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(web_sys::File::from);
                let mut new_readers = (*filereaders).clone();
                for file in files {
                    let filereader_id = filereader_ids.fetch_add(1, Ordering::SeqCst);
                    // let filereaders_for_remove = filereaders_for_remove.clone();
                    let filereader = gloo::file::callbacks::read_as_text(&file.into(), move |result| {
                        log!(result.unwrap());
                        log!(filereader_id);
                        let mut new_filereaders_for_remove = filereaders_for_remove.clone();
                        new_filereaders_for_remove.remove(&filereader_id);
                        // filereaders_for_remove.set(new_filereaders_for_remove);
                    });
                    new_readers.insert(filereader_id, Rc::new(filereader));
                }
                filereaders.set(new_readers);
            }
        })
    };

    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
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
