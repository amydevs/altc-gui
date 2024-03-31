use std::fmt::Display;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct FileInfo {
    pub name: String,
    pub contents: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub classes: Classes,
    pub file: FileInfo,
    #[prop_or_default]
    pub on_edit: Callback<()>,
}

#[function_component(File)]
pub fn file(props: &Props) -> Html {
    let active = use_state(|| false);
    html! {
        <div>
            // Heading
            <div>
                { &props.file.name }
            </div>
        </div>
    }
}