use chrono::{Datelike, Utc};
use yew::{function_component, html};

use crate::components::social_media_icons::SocialMediaIcons;

#[function_component(Footer)]
pub fn footer() -> Html {
    let year = get_current_year();
    html! {
        <footer>
            <SocialMediaIcons />
            <p
                class="text-center my-2 text-soft-black dark:text-white text-xs tracking-wide font-normal"
            >
                {format!("\u{00A9} 1993-{year} \u{2022} Tyler Earls")}
            </p>
        </footer>
    }
}

fn get_current_year() -> i32 {
    Utc::now().year()
}
