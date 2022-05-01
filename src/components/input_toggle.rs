use yew::{html, Component, Context, Html, Properties};

pub enum InputToggleMsg {
    InputToggled,
}

#[derive(Properties, PartialEq, Clone)]
pub struct InputToggleProps {
    #[prop_or(true)]
    toggled: bool,
    #[prop_or("input-toggle".to_string())]
    id: String,
    #[prop_or("input-toggle".to_string())]
    name: String,
}

pub struct InputToggle {
    toggled: bool,
    id: String,
    name: String,
}

impl Component for InputToggle {
    type Message = InputToggleMsg;
    type Properties = InputToggleProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            toggled: ctx.props().toggled,
            id: ctx.props().id.clone(),
            name: ctx.props().name.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            InputToggleMsg::InputToggled => {
                self.toggled = !self.toggled;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onclick = link.callback(|_| InputToggleMsg::InputToggled);
        html! {
            <div
                class="relative inline-block w-10 h-auto mr-2 align-middle select-none transition duration-800 ease-in"
                {onclick}
            >
                <input
                    type="checkbox"
                    class="input-toggle focus:outline-none dark:border-purple-400"
                    id={self.id.clone()}
                    name={self.name.clone()}   
                    checked={self.toggled}
                />
                <label
                    class="input-toggle-label block overflow-hidden h-5 rounded-full bg-gray-400 cursor-pointer"
                    for={self.id.clone()}
                />
            </div>
        }
    }
}
