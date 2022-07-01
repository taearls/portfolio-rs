use yew::{function_component, html};

use crate::components::Navigation;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="m-0 w-full">
            <Navigation />
        </header>
    }
}
