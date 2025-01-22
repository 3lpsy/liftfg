use tauri::{Builder, Wry};
use tauri_plugin_log::{Target, TargetKind};

pub fn setup(mut builder: Builder<Wry>) -> Builder<Wry> {
    let logging = tauri_plugin_log::Builder::new()
        .clear_targets()
        .target(Target::new(TargetKind::Stdout))
        .level(log::LevelFilter::Info)
        .level_for("tauri::plugin", log::LevelFilter::Warn)
        .level_for("tauri::app", log::LevelFilter::Warn)
        .level_for("tauri::manager", log::LevelFilter::Warn)
        .build();
    builder = builder.plugin(logging).plugin(tauri_plugin_opener::init());
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        builder = builder
            .plugin(tauri_plugin_cli::init())
            .plugin(tauri_plugin_fs::init());
    }
    builder // No need for return and semicolon
}
