use yew::{classes, function_component, html, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct InlineAnchorProps {
    #[prop_or_default]
    pub children: Children,
    pub aria_label: String,
    pub href: String,
    #[prop_or("p-0 m-0".to_string())]
    pub classes: String,
    #[prop_or(true)]
    pub is_external: bool,
}

#[function_component(InlineAnchor)]
pub fn inline_anchor(props: &InlineAnchorProps) -> Html {
    html! {
        if props.is_external {
            <ExternalLink children={props.children.clone()} aria_label={props.aria_label.clone()} href={props.href.clone()} classes={props.classes.clone()} />
        } else {
            <InternalLink children={props.children.clone()} aria_label={props.aria_label.clone()} href={props.href.clone()} classes={props.classes.clone()} />
        }
    }
}

#[function_component(InternalLink)]
fn internal_link(props: &InlineAnchorProps) -> Html {
    html! {
        <a
            class={classes!("inline-block text-lg font-extrabold text-purple-700 rounded-sm dark:text-purple-400 group-hover:text-cyan-400 dark:group-hover:text-cyan-300 sm:items-center sm:justify-center focus:outline-none focus:shadow-outline-light dark:focus:shadow-outline-dark".to_string(), props.classes.clone())}
            aria-label={props.aria_label.clone()}
            href={props.href.clone()}
        >
            {for props.children.iter()}
        </a>
    }
}

#[function_component(ExternalLink)]
fn external_link(props: &InlineAnchorProps) -> Html {
    html! {
        <span class="group inline-block">
            <a
                class={classes!("inline-block pr-6 text-lg font-extrabold text-purple-700 rounded-sm dark:text-purple-400 group-hover:text-cyan-400 dark:group-hover:text-cyan-300 sm:items-center sm:justify-center focus:outline-none focus:shadow-outline-light dark:focus:shadow-outline-dark".to_string(), props.classes.clone())}
                aria-label={props.aria_label.clone()}
                target="_blank"
                href={props.href.clone()}
            >
                <span>{for props.children.iter()}</span>
                <ExternalLinkIcon />
            </a>
        </span>
    }
}

#[function_component(ExternalLinkIcon)]
fn external_link_icon() -> Html {
    html! {
        <svg
            class="stroke-current fill-current mx-0.5 text-purple-700 dark:text-purple-400 group-hover:text-cyan-400 dark:group-hover:text-cyan-300 absolute inline-block"
            height="24px"
            width="24px"
            stroke-width="5"
            xmlns="http://www.w3.org/2000/svg"
            // xmlns:xlink="http://www.w3.org/1999/xlink"
            version="1.1"
            x="0px"
            y="0px"
            viewBox="0 0 100 100"
            style="margin-bottom: 0.1rem;"
            // xml:space="preserve"
            role="presentation"
            aria-labelledby="externalLinkIcon"
        >
            <title id="externalLinkIcon">{"External Link"}</title>
            <desc>{"Icon indicating the user will visit an external site in a separate tab."}</desc>
            <path
                d="M28.8,83.1h36l0,0c6.6,0,12-5.4,12-12v-22c0-1.1-0.9-2-2-2l0,0c-1.1,0-2,0.9-2,2v22c0,4.4-3.6,8-8,8l0,0h-36  c-4.4,0-8-3.6-8-8v-36c0-4.4,3.6-8,8-8l0,0h22l0,0c1.1,0,2-0.9,2-2s-0.9-2-2-2h-22l0,0c-6.6,0-12,5.4-12,12v36  C16.8,77.7,22.2,83.1,28.8,83.1z"
            />
            <path
                d="M83.2,37.2V18.9c0-0.1,0-0.3,0-0.4c0,0,0-0.1,0-0.1c0-0.1,0-0.2-0.1-0.3L83,18c0-0.1-0.1-0.2-0.1-0.2  c-0.1-0.2-0.3-0.4-0.6-0.6c-0.1-0.1-0.2-0.1-0.3-0.1H82L81.7,17h-0.1c-0.1,0-0.3,0-0.4,0l0,0H62.9l0,0c-1.1,0-2,0.9-2,2s0.9,2,2,2  h13.5L47.1,50.1c-0.8,0.8-0.8,2,0,2.8c0.8,0.8,2,0.8,2.8,0l29.3-29.2v13.5c0,1.1,0.9,2,2,2l0,0C82.3,39.2,83.2,38.3,83.2,37.2z"
            />
        </svg>
    }
}
