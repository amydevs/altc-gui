use yew::prelude::*;

pub mod file;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Html,
}


#[function_component(App)]
pub fn filelist(props: &Props) -> Html {
    html! {
        <div>
            {
                props.children.clone()
            }
        </div>
    }
}