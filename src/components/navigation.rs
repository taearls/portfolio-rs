use yew::{classes, function_component, html, Component, Context, Html, Properties};

use crate::components::InlineAnchor;

pub struct Navigation;

#[derive(Properties, PartialEq, Clone)]
struct NavigationLink {
    aria_label: String,
    display_text: String,
    href: String,
    #[prop_or_default]
    is_external: bool,
    is_last: bool,
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
                is_last: false,
            },
            NavigationLink {
                aria_label: "Visit Web Projects Page".to_string(),
                display_text: "Web".to_string(),
                href: "/web-projects".to_string(),
                is_external: false,
                is_last: false,
            },
            NavigationLink {
                aria_label: "Visit Contact Page".to_string(),
                display_text: "Contact".to_string(),
                href: "/contact".to_string(),
                is_external: false,
                is_last: false,
            },
            NavigationLink {
                aria_label: "Listen to Tyler's music on Bandcamp".to_string(),
                display_text: "Music".to_string(),
                href: "https://cuckooandthebirds.bandcamp.com".to_string(),
                is_external: true,
                is_last: true,
            },
        ];
        html! {
            <div class="w-full h-fit mb-4">
                <nav
                    class={"fixed bg-soft-black flex flex-col sm:flex-row items-center justify-evenly sm:justify-center w-full top-0 font-default font-mono dark:text-white text-black border border-b border-t-0 border-l-0 border-r-0 mb-4 sm:h-16 h-48"}
                >
                    <ul class="flex flex-col h-auto sm:h-16 items-center justify-center sm:flex-row w-40 sm:w-full">
                        {
                            links.into_iter().enumerate().map(|(index, link)| {
                                html! {
                                    <NavigationLinkListItem
                                        is_last={link.is_last}
                                        key={index}
                                        aria_label={link.aria_label}
                                        is_external={link.is_external}
                                        href={link.href}
                                        display_text={link.display_text}
                                    />
                                }
                            }).collect::<Html>()
                        }
                    </ul>
                </nav>
                <div
                    class={"w-screen top-0 bg-none sm:h-16 h-48"}
                >
                </div>
            </div>
        }
    }
}

#[function_component(NavigationLinkListItem)]
fn navigation_link_list_item(link: &NavigationLink) -> Html {
    let border = if link.is_last {
        "border-none"
    } else {
        "border border-gray-400 dark:border-gray-500 border-t-0 border-l-0 border-r-0 border-b-1 sm:border-none"
    };
    html! {
        <li
            class={classes!("flex justify-center sm:inline-block mx-auto py-2 sm:py-0 w-full sm:mx-0 sm:w-auto sm:h-fit".to_string(), border)}
        >
            <InlineAnchor
                classes={"m-0 px-4"}
                aria_label={link.aria_label.clone()}
                href={link.href.clone()}
                is_external={link.is_external}
            >
                {link.display_text.clone()}
            </InlineAnchor>
        </li>
    }
}
