use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct IconProps {
    pub svg_class: Option<String>,
    pub path_class: Option<String>,
}

#[function_component(PlayIcon)]
pub fn play_icon(props: &IconProps) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" class={classes!(props.svg_class.clone())} viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" class={classes!(props.path_class.clone())} d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z"  clip-rule="evenodd" />
        </svg>
    }
}

#[function_component(GearIcon)]
pub fn gear_icon(props: &IconProps) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" class={classes!(props.svg_class.clone())} viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" class={classes!(props.path_class.clone())} d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z"  clip-rule="evenodd" />
        </svg>
    }
}

#[function_component(RefreshIcon)]
pub fn refresh_icon(props: &IconProps) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" class={classes!(props.svg_class.clone())} viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" class={classes!(props.path_class.clone())} d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd" />
        </svg>
    }
}

#[function_component(TrashIcon)]
pub fn trash_icon(props: &IconProps) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" class={classes!(props.svg_class.clone())} fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" class={classes!(props.path_class.clone())} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
        </svg>
    }
}
