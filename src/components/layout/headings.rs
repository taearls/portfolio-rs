use std::fmt::{Display, Formatter};
use yew::{classes, function_component, html, Children, Classes, Properties};

#[derive(PartialEq)]
pub enum HeadingAlignment {
    Left,
    Center,
    #[allow(dead_code)]
    Right,
}

impl Display for HeadingAlignment {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use HeadingAlignment::*;
        let class: &str = match self {
            Left => "left",
            Center => "center",
            Right => "right",
        };
        write!(f, "text-{class}")
    }
}

#[derive(Properties, PartialEq)]
pub struct HeadingProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(HeadingAlignment::Center)]
    pub align: HeadingAlignment,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: String,
}

#[function_component(HeadingOne)]
pub fn heading_one(props: &HeadingProps) -> Html {
    let HeadingProps {
        children,
        align,
        class,
        style,
    } = props;
    html! {
        <h1
            class={classes!(class.clone(), format!("{align}"), "mb-4 text-purple-700 dark:text-purple-400 font-extrabold text-4xl".to_string())}
            style={style.clone()}
        >
            {for children.iter()}
        </h1>
    }
}

#[function_component(HeadingTwo)]
pub fn heading_two(props: &HeadingProps) -> Html {
    let HeadingProps {
        children,
        align,
        class,
        style,
    } = props;
    html! {
        <h2
            class={classes!(class.clone(), format!("{align}"), "font-bold text-3xl mb-8 text-purple-700 dark:text-purple-400".to_string())}
            style={style.clone()}
        >
            {for children.iter()}
        </h2>
    }
}
