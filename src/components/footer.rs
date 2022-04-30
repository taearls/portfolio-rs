use yew::{html, Component, Context, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct FooterProps {
    pub year: i32,
}

pub struct Footer;

impl Component for Footer {
    type Properties = FooterProps;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <footer>
                <p
                    class="text-center text-soft-black dark:text-white text-xs tracking-wide font-normal"
                >
                    {'\u{00A9}'}{'\u{00a0}'}{ctx.props().year}
                </p>
            </footer>
        }
    }
}
