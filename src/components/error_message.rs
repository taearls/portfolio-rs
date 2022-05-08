use yew::{classes, function_component, html, Children, Classes, Properties};

#[derive(Properties, PartialEq)]
pub struct ErrorMessageProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub show: bool,
}

#[function_component(ErrorMessage)]
pub fn error_message(props: &ErrorMessageProps) -> Html {
    let ErrorMessageProps {
        children,
        class,
        style,
        show,
    } = props;
    html! {
        if *show {
            <p
                class={classes!(class.clone(), "text-red-600 dark:text-red-500 font-semibold".to_string())}
                style={style.clone()}
            >
                {for children.iter()}
            </p>
        }
    }
}
