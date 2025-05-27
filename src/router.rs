use crate::layouts::*;
use crate::pages::Home;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/home")]
    Workflows,

    #[at("/some/page-2")]
    Page2,

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(NotFound)]
fn not_found() -> Html {
    html! {
        <h1>{ "404" }</h1>
    }
}

#[function_component(Page2)]
fn page_2() -> Html {
    html! {
    <>
        <PageHeader title="Page 2"  />
        <Section title="Section">
            <Link<Route> to={Route::Workflows{}}>
                    {"Home"}
            </Link<Route>>
        </Section>

    </>
    }
}

#[derive(Properties, PartialEq)]
struct RootProps {
    component: Html,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Width {
    width: i32,
}

#[function_component[Root]]
fn root(props: &RootProps) -> Html {
    let state = use_window_size();

    let toggle_menu = use_state(|| false);

    let toggle = {
        let toggle_menu = toggle_menu.clone();
        Callback::from(move |_| {
            toggle_menu.set(!*toggle_menu);
        })
    };

    let container_classes = if *toggle_menu {
        "ccontainer page-wrapper row"
    } else {
        if state.0 <= 768. {
            "ccontainer page-wrapper row sidebar-open"
        } else {
            "ccontainer page-wrapper row"
        }
    };

    let sidebar_classes = if *toggle_menu {
        "sidebar sidebar-hide"
    } else {
        "sidebar sidebar-show"
    };

    // Use bootstrap sizes instead of picking my own
    let (dynamic_menu_classes, dynamic_content_classes) = if *toggle_menu {
        ("", "main col col-xl-12 col-lg-12 col-md-12 col-sm-12")
    } else {
        (
            "col col-xl-2 col-lg-3 col-md-3 col-sm-12",
            "main col col-xl-10 col-lg-9 col-md-9 col-sm-12",
        )
    };

    let blur = if *toggle_menu {
        "overlay"
    } else {
        if state.0 <= 768. {
            "overlay visible"
        } else {
            "overlay"
        }
    };

    html! {
    <>
        <div class={container_classes}>

            <div class="toggle-btn" menu-open={format!("{}",*toggle_menu)}>
                <button type="button" onclick={toggle.clone()} data-bs-toggle="button" class="btn btn-outline-dark btn-menu-toggle">{"☰"}</button>
            </div>

            <div class={dynamic_menu_classes}>
                <nav class={sidebar_classes}>
                    <PageHeader title="Yew Admin" />
                    <div class="scrollbox navbar-wrapper">
                        <div class="scrollbox-inner">
                        <ul>
                            <li>
                                <Link<Route> to={Route::Page2{}}>
                                    {"Page 2"}
                                </Link<Route>>
                            </li>
                            {(1..=100).map(|i| html! {<li>{"Link #"}{i}</li>}).collect::<Html>()}
                        </ul>
                        </div>
                    </div>
                </nav>
            </div>
            <div class={dynamic_content_classes}>
                <section>
                    <div class="page-content">
                        {props.component.clone()}
                    </div>
                </section>
            </div>
        </div>

        <div class={blur} onclick={toggle.clone()}></div>
    </>
    }
}

pub fn switch(routes: Route) -> Html {
    let component = match &routes {
        Route::Home => html! { <Home /> },
        Route::Workflows => html! { <Home /> },
        Route::Page2 => html! { <Page2 /> },
        Route::NotFound => html! { <NotFound /> },
    };
    html! {
        <>
            <Root component={component} />
        </>
    }
}
