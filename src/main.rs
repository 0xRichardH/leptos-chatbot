use cfg_if::cfg_if;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        routing::{get, post},
        Router,
    };
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_chatbot::app::*;
    use leptos_chatbot::fileserv::file_and_error_handler;
    use leptos_chatbot::state::AppState;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    let state = AppState::new(leptos_options);

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

cfg_if! {
    if #[cfg(feature="ssr")] {
        use axum::{
            response::{Response, IntoResponse},
            extract::{Path, State, RawQuery},
            http::{Request, header::HeaderMap},
            body::Body as AxumBody,
        };
        use leptos_axum::handle_server_fns_with_context;
        use leptos::{log, view, provide_context};
        use leptos_chatbot::state::AppState;
        use leptos_chatbot::app::*;

        async fn server_fn_handler(State(app_state): State<AppState>, path: Path<String>, headers: HeaderMap, raw_query: RawQuery, request: Request<AxumBody>) -> impl IntoResponse {

            log!("{:?}", path);

            handle_server_fns_with_context(path, headers, raw_query, move |cx| {
                provide_context(cx, app_state.clone());
            }, request).await
        }

        async fn leptos_routes_handler(State(app_state): State<AppState>, req: Request<AxumBody>) -> Response{
                let handler = leptos_axum::render_app_to_stream_with_context(app_state.leptos_options.clone(),
                move |cx| {
                    provide_context(cx, app_state.clone());
                },
                |cx| view! { cx, <App/> }
            );
            handler(req).await.into_response()
        }
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
