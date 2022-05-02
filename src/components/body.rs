use yew::{html, Children, Component, Context, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct BodyProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct Body;

impl Component for Body {
    type Properties = BodyProps;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // rather than styling the body directly (api will be removed soon), I'm using this div to wrap the entire app
        // https://github.com/yewstack/yew/pull/2346
        html! {
            <div class="min-h-screen bg-soft-black w-full flex flex-col items-center gap-y-4">
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
