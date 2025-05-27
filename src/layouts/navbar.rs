use crate::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub extra_classes: String,
}

#[function_component(Navbar)]
pub fn navbar(_props: &Props) -> Html {
    html! {
    <>
        <nav class="navbar px-4" data-bs-theme="dark" style="background-color: #6f42c1;">
            <div class="" >
                <Link<Route> to={Route::Workflows } classes={"no-decoration"}>
                    <h2>
                        {"Yew Admin"}
                    </h2>
                </Link<Route>>
            </div>
        </nav>
    </>
    }
}

#[derive(Properties, PartialEq)]
pub struct HProps {
    #[prop_or(String::from("light"))]
    pub nav_theme: String,
    #[prop_or(String::from("Yew Admin"))]
    pub product: String,
    #[prop_or_default]
    pub current: String,
}

#[function_component(Header)]
pub fn header(props: &HProps) -> Html {
    let _ = props;

    let navbar_theme = &props.nav_theme;
    let logo_color = if navbar_theme == "dark" {
        "text-white"
    } else {
        "text-black"
    };

    let logo_color_accent = if navbar_theme == "dark" {
        "text-white"
    } else {
        "text-primary"
    };

    html! {
    <>
        <header class={format!("site-header navbar-{navbar_theme}")}>
            <div id="header-wrap" class="position-absolute w-100 z-index-1">
                <div class="container">
                    <div class="row">

                    <div class="col">
                        <nav class="navbar navbar-expand-lg">
                        <a class={format!("navbar-brand logo {logo_color} h2 mb-0")} href="/">
                            {&props.product}<span class={format!("fw-bold {logo_color_accent}")}>{" Stella"}</span>
                        </a>
                        <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav"
                            aria-expanded="false" aria-label="Toggle navigation"> <span class="navbar-toggler-icon"></span>
                        </button>
                        <div class="collapse navbar-collapse" id="navbarNav"></div>
                        <a class="btn btn-dark ms-8 d-none d-md-block" href="#">{"Login"}</a>
                        </nav>
                    </div>
                    </div>
                </div>
            </div>
        </header>
    </>
    }
}
