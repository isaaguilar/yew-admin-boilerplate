use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SectionProps {
    pub children: Children,
    #[prop_or_default]
    pub title: String,

    #[prop_or_default]
    pub raw_title: Html,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub extra_classes: String,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    html! {
        <div id={props.id.clone()} class={format!("row pb-5 px-4 {}", props.extra_classes)} >
            <div class="col p-4 bg-body border border-2 rounded rounded-2 shadow shadow-sm">
                <div class="row pb-3">
                    {
                        if props.title == String::new() {
                            props.raw_title.clone()
                        } else {
                            html! {
                                <div class="col">
                                    <h3>{format!("{}", props.title)}</h3>
                                </div>
                            }
                        }
                    }
                </div>
                <div class="container-fluid col">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
