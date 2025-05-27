use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ContentWrapperProps {
    pub children: Children,

    #[prop_or_default]
    pub extra_classes: String,
}

#[function_component(ContentWrapper)]
pub fn content_wrapper(props: &ContentWrapperProps) -> Html {
    html! {
    <div class="min-vh-100">
        <div class={format!("row {}", props.extra_classes)}>
            { for props.children.iter() }
        </div>
    </div>
    }
}
