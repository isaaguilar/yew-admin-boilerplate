use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    #[prop_or_default]
    pub subtitle: String,
    #[prop_or_default]
    pub raw_subtitle: Html,
}

#[function_component(PageHeader)]
pub fn page_header(props: &Props) -> Html {
    html! {
    <>
        <div class="row pt-2 pb-3">
            <h3 class="display-6">{props.title.clone()}</h3>
            {
                if props.subtitle == String::new() {
                    props.raw_subtitle.clone()
                } else {
                    html! {<h6>{props.subtitle.clone()}</h6>}
                }
            }
        </div>
    </>
    }
}

