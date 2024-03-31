use altc::util::LiveVersion;
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(LiveVersion::Live12)]
    pub value: LiveVersion,
}



#[function_component(LiveVersionSelectOptions)]
pub fn live_version_select_options(props: &Props) -> Html {
    let versions: &[LiveVersion] = &[
        LiveVersion::Live10,
        LiveVersion::Live11,
        LiveVersion::Live12,
    ];
    html! {
        <>
            {
                versions.iter().map(|version| {
                    let version_number = (*version as u8).to_string();
                    html! {
                        <option value={version_number.clone()} selected={ props.value == *version }>{ &format!("Live {}", version_number) }</option>
                    }
                }).collect::<Vec<_>>()
            }
        </>
    }
}