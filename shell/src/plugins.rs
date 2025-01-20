use tauri::{Builder, Wry};

pub fn setup(mut builder: Builder<Wry>) -> Builder<Wry> {
    builder = builder.plugin(tauri_plugin_opener::init());
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        builder = builder
            .plugin(tauri_plugin_cli::init())
            .plugin(tauri_plugin_fs::init());
    }
    builder // No need for return and semicolon
}
