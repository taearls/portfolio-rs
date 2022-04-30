use yew::{html, Component, Context, Html, Properties};

pub enum InputToggleMsg {
    InputToggled,
}

#[derive(Properties, PartialEq, Clone)]
pub struct InputToggleProps {
    #[prop_or(true)]
    toggled: bool,
}

pub struct InputToggle {
    toggled: bool,
}

impl Component for InputToggle {
    type Message = InputToggleMsg;
    type Properties = InputToggleProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            toggled: ctx.props().toggled,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            InputToggleMsg::InputToggled => {
                self.toggled = !self.toggled;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // TODO: finish translating this vue component
        let link = ctx.link();

        let onchange = link.callback(|_| InputToggleMsg::InputToggled);
        html! {
            <div
                class="relative inline-block w-10 h-auto mr-2 align-middle select-none transition duration-800 ease-in"
            >
                <input
                    id="darkModeToggle"
                    type="checkbox"
                    // :checked="toggledOn"
                    // name="darkModeToggle"
                    class="toggle-checkbox focus:outline-none dark:border-purple-400"
                    {onchange}
                />
                <label
                    for="darkModeToggle"
                    class="toggle-label block overflow-hidden h-5 rounded-full bg-gray-400 cursor-pointer"
                />
            </div>
        }
    }
}
