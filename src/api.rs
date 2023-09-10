use cfg_if::cfg_if;
use leptos::*;

use crate::model::conversation::Conversation;
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::state::AppState;
    }
}

#[server(Converse, "/api")]
pub async fn converse(cx: Scope, prompt: Conversation) -> Result<String, ServerFnError> {
    let state = use_context::<AppState>(cx)
        .ok_or(ServerFnError::ServerError("No app state".to_string()))?;

    println!("{:?}", prompt);
    Ok(String::from("hello"))
}
