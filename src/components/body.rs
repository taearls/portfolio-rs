use yew::{function_component, html, Children, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct BodyProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Body)]
pub fn body(props: &BodyProps) -> Html {
    // rather than styling the body directly (api will be removed soon), I'm using this div to wrap the entire app
    // https://github.com/yewstack/yew/pull/2346
    let BodyProps { children } = props;
    html! {
        <div class="min-h-screen bg-soft-black w-full overflow-hidden">
            { for children.iter() }
        </div>
    }
}
