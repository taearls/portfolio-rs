extern crate reqwest_wasm;

use yew::prelude::*;

mod components;
use components::{Footer, InputToggle};

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="h-screen bg-gray-600 w-full flex flex-col items-center justify-center gap-y-4">
            <InputToggle />
            <Footer />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
