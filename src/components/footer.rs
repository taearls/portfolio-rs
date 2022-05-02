use chrono::{Datelike, Utc};
use yew::{html, Component, Context, Html};

use super::social_media_icons::SocialMediaIcons;
pub struct Footer {
    year: i32,
}

fn get_current_year() -> i32 {
    Utc::now().year()
}

impl Component for Footer {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            year: get_current_year(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <footer>
                <SocialMediaIcons />
                <p
                    class="text-center my-2 text-soft-black dark:text-white text-xs tracking-wide font-normal"
                >
                    {format!("\u{00A9} 1993-{} \u{2022} Tyler Earls", self.year)}
                </p>
            </footer>
        }
    }
}
