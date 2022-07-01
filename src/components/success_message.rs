use yew::{classes, function_component, html, Children, Classes, Properties};

#[derive(Properties, PartialEq)]
pub struct SuccessMessageProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub show: bool,
}

#[function_component(SuccessMessage)]
pub fn error_message(props: &SuccessMessageProps) -> Html {
    let SuccessMessageProps {
        children,
        class,
        style,
        show,
    } = props;
    html! {
        if *show {
            <p
                class={classes!(class.clone(), "text-emerald-500 dark:text-emerald-400 font-semibold".to_string())}
                style={style.clone()}
            >
                {for children.iter()}
            </p>
        }
    }
}
