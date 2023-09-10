use leptos::*;

use crate::model::conversation::Conversation;

const CHAT_AREA_CLASS: &str = "h-screen pb-24 w-full flex flex-col overflow-y-auto p-5";
const CHAT_AREA_DARK_MODE_COLORS: &str = "border-zinc-700 bg-zinc-900";

const USER_MESSAGE_DARK_MODE_COLORS: &str = "bg-blue-500 text-white";
const USER_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-end";

const MODEL_MESSAGE_DARK_MODE_COLORS: &str = "bg-zinc-700 text-white";
const MODEL_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-start";

#[component]
pub fn ChatArea(cx: Scope, conversation: ReadSignal<Conversation>) -> impl IntoView {
    // let chat_div_ref = create_node_ref::<Div>(cx);

    create_effect(cx, move |_| conversation.get());

    view! {
        cx,
        <div class={format!("{CHAT_AREA_CLASS} {CHAT_AREA_DARK_MODE_COLORS}")} >
        {move || conversation.get().messages.iter().map(move |message| {
            let class_str = if message.user {
                format!("{USER_MESSAGE_CLASS} {USER_MESSAGE_DARK_MODE_COLORS}")
            } else {
                format!("{MODEL_MESSAGE_CLASS} {MODEL_MESSAGE_DARK_MODE_COLORS}")
            };
            view!{
                cx,
                <div class={class_str}>
                  {message.text.clone()}
                </div>
            }
         }).collect::<Vec::<_>>()
        }
        </div>
    }
}
