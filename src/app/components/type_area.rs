use leptos::{html::Input, *};

const TYPE_AREA_CLASS: &str =
    "h-24 w-full fixed bottom-0 flex justify-center items-center p-5 border-t";
const TYPE_AREA_CLASS_DARK: &str = "bg-zinc-900 border-zinc-700";

const TEXT_AREA_CLASS: &str = "w-2/3 p-4 border rounded-full input-field";
const TEXT_AREA_CLASS_DARK: &str = "bg-zinc-700 border-zinc-700 text-white";

const BUTTON_CLASS: &str = "h-full p-4 rounded-full cursor-pointer";
const BUTTON_CLASS_DARK: &str = "bg-green-700 text-white";

#[component]
pub fn TypeArea(
    cx: Scope,
    send_message: Action<String, Result<String, ServerFnError>>,
) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    view! {
        cx,
         <div class={format!("{TYPE_AREA_CLASS} {TYPE_AREA_CLASS_DARK}")}>
           <form class="w-full flex justify-center items-center gap-4" on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                send_message.dispatch(input.value());
                input.set_value("");
           }
           >
                <input class={format!("{TEXT_AREA_CLASS} {TEXT_AREA_CLASS_DARK}")} type="text" placeholder="Enter your prompt" node_ref=input_ref/>
                <button class={format!("{BUTTON_CLASS} {BUTTON_CLASS_DARK}")} type="submit">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12h15m0 0l-6.75-6.75M19.5 12l-6.75 6.75" />
                    </svg>
                </button>
           </form>
        </div>
    }
}
