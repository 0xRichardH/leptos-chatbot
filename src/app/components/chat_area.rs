use leptos::*;

const CHAT_AREA_CLASS: &str = "h-screen pb-24 w-full flex flex-col overflow-y-auto p-5";
const CHAT_AREA_DARK_MODE_COLORS: &str = "border-zinc-700 bg-zinc-900";

#[component]
pub fn ChatArea(cx: Scope) -> impl IntoView {
    // let chat_div_ref = create_node_ref::<Div>(cx);

    view! {
        cx,
        <div class={format!("{CHAT_AREA_CLASS} {CHAT_AREA_DARK_MODE_COLORS}")} >
            <div>Hello</div>
        </div>
    }
}
