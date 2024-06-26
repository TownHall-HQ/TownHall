use leptos::*;

#[slot]
pub struct Fallback {
    children: ChildrenFn,
}

#[component]
pub fn MaybeChildren<T, CF, IV>(
    value: Option<T>,
    children: CF,
    #[prop(optional)] fallback: Option<Fallback>,
) -> impl IntoView
where
    CF: FnOnce(T) -> IV + 'static,
    IV: IntoView,
{
    if let Some(value) = value {
        children(value).into_view()
    } else if let Some(fallback) = fallback {
        (fallback.children)().into_view()
    } else {
        ().into_view()
    }
}
