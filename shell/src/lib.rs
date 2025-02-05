#![feature(stmt_expr_attributes)]

use fgcore::logging;
use tracing::info;
mod config;
pub mod handlers;
mod plugins;
mod setup;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Only env that should be read before setup is RUST_LOG
    // Revisit if there are tauri runtime variables needed
    let log_handles = match logging::init() {
        Ok(handles) => {
            info!("Console tracing initialized");
            handles
        }
        Err(e) => {
            panic!("Error setting up logging: {:?}", e);
        }
    };
    let mut builder = tauri::Builder::default();
    builder = plugins::load(builder);

    // Setup Callback
    builder = builder.setup(|app| Ok(setup::setup(app, log_handles, None)?));
    builder = builder.invoke_handler(handlers::generate());

    builder.run(tauri::generate_context!()).unwrap_or_else(|e| {
        eprintln!("Error while running Tauri application: {:?}", e);
        panic!("Error while running Tauri application")
    });
}

#[cfg(test)]
pub mod testutils {
    use crate::config::AppConfig;
    use crate::setup;
    use fgutils;

    use super::{handlers, plugins};
    use anyhow::{anyhow, Result};
    use ctor::ctor;
    use fgcore::logging;
    use std::fs;
    use std::sync::OnceLock;
    use tauri::test::{mock_builder, MockRuntime};
    use tauri::{generate_context, App, WebviewWindow, WebviewWindowBuilder};
    use uuid::Uuid;

    static INIT: OnceLock<()> = OnceLock::new();
    static DATA_DIR: OnceLock<std::path::PathBuf> = OnceLock::new();
    pub static LOGGING_HANDLES: OnceLock<(logging::LayersHandle, logging::FilterHandle)> =
        OnceLock::new();

    #[ctor]
    fn init_tests() {
        INIT.get_or_init(|| {
            let data_dir = fgutils::cwd().join("data").join("tests");
            if !data_dir.exists() {
                fs::create_dir_all(&data_dir).expect("Could not create testing data dir");
            }
            DATA_DIR.set(data_dir).unwrap();
            std::env::set_var(
                "XDG_DATA_HOME",
                DATA_DIR.get().unwrap().to_string_lossy().to_string(),
            );
            std::env::set_var("APP_ENV", "test");
            std::env::set_var("NO_DOTENV", "true");
            let handles = logging::init().unwrap();
            match LOGGING_HANDLES.set(handles) {
                Ok(_) => Ok(()),
                Err(_e) => Err(anyhow!("Failed to set logging handles on lock")),
            }
            .unwrap();
        });
    }

    pub fn create_config(test_id: Uuid) -> AppConfig {
        let mut config = AppConfig::default(&data_dir(test_id));
        config.no_fs_logging = true;
        config.no_dotenv = true;
        config.no_logging_filer_reload = true;
        config
    }

    // // TOOD: create testing json file and just load that
    // pub fn create_context<R: Runtime>() -> Context<R> {
    //     // let mut plugins_config = HashMap::new();
    //     // let cli_config = json!({
    //     //     "description": "Test CLI",
    //     //     "longDescription": "Test CLI for unit tests",
    //     //     "beforeHelp": "",
    //     //     "afterHelp": "",
    //     //     "args": [],
    //     //     "subcommands": {}
    //     // });
    //     // plugins_config.insert("cli".to_string(), cli_config);
    //     // let mut context = mock_context(noop_assets());
    //     // let config = context.config_mut();
    //     // config.plugins = PluginConfig(plugins_config);
    //     // context
    //     //     .runtime_authority
    //     //     .add_capability("log")
    //     //     .expect("Failed to add log capability");
    //     context
    // }
    pub fn data_dir(test_id: Uuid) -> std::path::PathBuf {
        DATA_DIR.get().unwrap().join(test_id.to_string())
    }
    pub fn create_app() -> Result<App<MockRuntime>> {
        let mut builder = mock_builder();
        builder = plugins::load(builder);
        let app = builder
            .setup(|_app| Ok(()))
            .invoke_handler(handlers::generate())
            .build(generate_context!("test.tauri.conf.json"))?;
        Ok(app)
    }

    pub fn create_webview(app: &App<MockRuntime>) -> Result<WebviewWindow<MockRuntime>> {
        let webview = WebviewWindowBuilder::new(app, "main", Default::default()).build()?;
        Ok(webview)
    }

    pub fn create_app_and_webview() -> Result<(App<MockRuntime>, WebviewWindow<MockRuntime>)> {
        let app = create_app()?;
        let webview = create_webview(&app)?;
        Ok((app, webview))
    }

    pub async fn default_test_setup() -> Result<(App<MockRuntime>, WebviewWindow<MockRuntime>, Uuid)>
    {
        let test_id = Uuid::new_v4();
        let (mut app, webview) = create_app_and_webview().unwrap();
        setup::setup_async(
            &mut app,
            LOGGING_HANDLES.get().unwrap().to_owned(),
            Some(create_config(test_id)),
        )
        .await
        .unwrap();
        Ok((app, webview, test_id))
    }
}
