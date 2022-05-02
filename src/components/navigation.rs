use yew::{function_component, html, Component, Context, Html, Properties};

pub struct Navigation;

#[derive(Properties, PartialEq, Clone)]
struct NavigationLink {
    aria_label: String,
    display_text: String,
    href: String,
    #[prop_or_default]
    is_external: bool,
}

impl Component for Navigation {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let links = vec![
            NavigationLink {
                aria_label: "Visit Home Page".to_string(),
                display_text: "Home".to_string(),
                href: "/".to_string(),
                is_external: false,
            },
            NavigationLink {
                aria_label: "Visit Web Projects Page".to_string(),
                display_text: "Web".to_string(),
                href: "/web-projects".to_string(),
                is_external: false,
            },
            NavigationLink {
                aria_label: "Visit Contact Page".to_string(),
                display_text: "Contact".to_string(),
                href: "/contact".to_string(),
                is_external: false,
            },
            NavigationLink {
                aria_label: "Listen to Tyler's music on Bandcamp".to_string(),
                display_text: "Music".to_string(),
                href: "https://cuckooandthebirds.bandcamp.com".to_string(),
                is_external: true,
            },
        ];
        html! {
            <div>
                <ul class="flex flex-col h-auto justify-center sm:flex-row sm:justify-end">
                    {
                        links.into_iter().enumerate().map(|(index, link)| {
                            html! {
                                <li
                                    class="flex justify-center sm:inline-block mx-auto py-2 text-center w-1/3 border border-gray-400 dark:border-gray-500 border-t-0 border-l-0 border-r-0 border-b-1 sm:border-none sm:mx-0 sm:w-auto"
                                    key={index}
                                >
                                    if link.is_external {
                                        <ExternalLink aria_label={link.aria_label} display_text={link.display_text} href={link.href} />
                                    } else {
                                        <InternalLink aria_label={link.aria_label} display_text={link.display_text} href={link.href} />
                                    }
                                </li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            </div>
        }
    }
}

#[function_component(InternalLink)]
fn internal_link(link: &NavigationLink) -> Html {
    html! {
        <a
            class="inline-block pl-6 pr-12 sm:pl-4 sm:pr-2 text-lg font-extrabold text-purple-700 rounded-sm dark:text-purple-400 dark:hover:text-cyan-300 sm:flex sm:items-center sm:justify-center focus:outline-none focus:shadow-outline-light dark:focus:shadow-outline-dark"
            aria-label={link.aria_label.clone()}
            href={link.href.clone()}
        >
            {link.display_text.clone()}
        </a>
    }
}

#[function_component(ExternalLink)]
fn external_link(link: &NavigationLink) -> Html {
    html! {
        <div class="group">
            <a
                class="inline-block pl-6 pr-12 sm:pl-4 sm:pr-2 text-lg font-extrabold text-purple-700 rounded-sm dark:text-purple-400 group-hover:text-cyan-400 dark:group-hover:text-cyan-300 sm:flex sm:items-center sm:justify-center focus:outline-none focus:shadow-outline-light dark:focus:shadow-outline-dark"
                aria-label={link.aria_label.clone()}
                target="_blank"
                href={link.href.clone()}
            >
                {link.display_text.clone()}
                <svg
                    class="stroke-current fill-current text-purple-700 dark:text-purple-400 group-hover:text-cyan-400 dark:group-hover:text-cyan-300 absolute inline-block ml-1 sm:static sm:mx-1"
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
            </a>
        </div>
    }
}
