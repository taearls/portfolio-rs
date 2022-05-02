extern crate reqwest_wasm;

use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::{Body, Footer};

mod pages;
use pages::{Home, Web, Contact, NotFound};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/web-projects")]
    Web,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Web => html! { <Web /> },
        Route::Contact => html! { <Contact /> },
        Route::NotFound => html! { <NotFound />}
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Body>
                <Switch<Route> render={Switch::render(switch)} />
                <Footer />
            </Body>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
