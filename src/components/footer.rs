use chrono::{Datelike, Utc};
use yew::{html, Component, Context, Html};

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
                <p
                    class="text-center text-soft-black dark:text-white text-xs tracking-wide font-normal"
                >
                    {'\u{00A9}'}{'\u{00a0}'}{self.year}
                </p>
            </footer>
        }
    }
}
