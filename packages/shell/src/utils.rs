// TODO: to reduce overhead, just create a default db and copy the file
#[cfg(test)]
pub mod testutils {
    use crate::config::AppConfig;
    use crate::setup;
    use fgdb::data::{ResponsableData, ResponseData};
    use fgutils;
    use tauri::ipc::InvokeBody;

    use crate::{commands, plugins};
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
            let data_dir = fgutils::cwd().join("appdata").join("tests");
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

    pub async fn invoke<T>(
        webview: &WebviewWindow<MockRuntime>,
        cmd: &str,
        body: InvokeBody,
    ) -> ResponseData<T>
    where
        T: ResponsableData,
    {
        match tauri::test::get_ipc_response(
            webview,
            tauri::webview::InvokeRequest {
                cmd: cmd.into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: "tauri://localhost".parse().unwrap(),
                body,
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .map(|b| b.deserialize::<ResponseData<T>>().unwrap())
        .map_err(|b| serde_json::from_value::<ResponseData<T>>(b).unwrap())
        {
            Ok(r) => return r,
            Err(e) => return e,
        }
    }

    pub fn create_config(test_id: Uuid) -> AppConfig {
        let mut config = AppConfig::default(&data_dir(test_id));
        config.no_fs_logging = true;
        config.no_dotenv = true;
        config.no_logging_filer_reload = true;
        config
    }
    pub fn data_dir(test_id: Uuid) -> std::path::PathBuf {
        DATA_DIR.get().unwrap().join(test_id.to_string())
    }
    pub fn create_app() -> Result<App<MockRuntime>> {
        let mut builder = mock_builder();
        builder = plugins::load(builder);
        let app = builder
            .setup(|_app| Ok(()))
            .invoke_handler(commands::generate())
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
    pub async fn seeded_dev_test_setup(
    ) -> Result<(App<MockRuntime>, WebviewWindow<MockRuntime>, Uuid)> {
        let test_id = Uuid::new_v4();
        let mut config = create_config(test_id);
        config.should_seed_dev = true;
        let (mut app, webview) = create_app_and_webview().unwrap();
        setup::setup_async(
            &mut app,
            LOGGING_HANDLES.get().unwrap().to_owned(),
            Some(config),
        )
        .await
        .unwrap();
        Ok((app, webview, test_id))
    }
}
