mod components;

use crate::{
    api::converse,
    app::components::chat_area::ChatArea,
    app::components::type_area::TypeArea,
    error_template::{AppError, ErrorTemplate},
    model::conversation::{Conversation, Message},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-chatbot.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let (conversation, set_conversation) = create_signal(cx, Conversation::new());
    let send_message = create_action(cx, move |new_message: &String| {
        let message = Message {
            user: true,
            text: new_message.clone(),
        };
        set_conversation.update(move |c| {
            c.messages.push(message);
        });

        converse(cx, conversation.get())
    });

    // display `...` when submmiting message to the server
    create_effect(cx, move |_| {
        if send_message.input().get().is_some() {
            let model_msg = Message {
                user: false,
                text: String::from("..."),
            };
            set_conversation.update(move |c| {
                c.messages.push(model_msg);
            });
        }
    });

    // display the resposne from the server
    create_effect(cx, move |_| {
        if let Some(Ok(msg)) = send_message.value().get() {
            set_conversation.update(move |c| {
                if let Some(last_msg) = c.messages.last_mut() {
                    last_msg.text = msg;
                }
            });
        }
    });

    view! { cx,
        <ChatArea conversation/>
        <TypeArea send_message/>
    }
}
