use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")]{
        use llm::models::Llama;
        use dotenv::dotenv;
        use std::sync::Arc;
        use std::env;
        use leptos::LeptosOptions;
        use axum::extract::FromRef;

        #[derive(Clone, FromRef)]
        pub struct AppState {
            pub leptos_options: LeptosOptions,
            pub language_model: Arc<Llama>,
        }

        impl AppState {
            pub fn new(leptos_options: LeptosOptions) -> Self {
                let language_model = Arc::new(get_langugae_model());
                Self {
                    leptos_options,
                    language_model
                }
            }
        }

        fn get_langugae_model() -> Llama {
            dotenv().ok();
            let model_path = env::var("MODEL_PATH").expect("MODEL_PATH must be set");
            let model_parameters = llm::ModelParameters {
                prefer_mmap: true,
                context_size: 2048,
                lora_adapters: None,
                use_gpu: true,
                gpu_layers: None,
                rope_overrides: None,
                n_gqa: None,
            };

            llm::load::<Llama>(
              &std::path::PathBuf::from(&model_path),
              llm::TokenizerSource::Embedded,
              model_parameters,
              llm::load_progress_callback_stdout,
            ).unwrap_or_else(|err|{
                panic!("Failed to load language model from {model_path:?}: {err}");
            })
        }
    }
}
