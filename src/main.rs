mod layouts;
mod pages;
mod router;
mod store;
mod util;

use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

const API_ENDPOINT: &str = if cfg!(debug_assertions) {
    // Development environment
    "http://localhost:3000"
} else {
    ""
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter basename="/yew-admin-boilerplate">
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
