use tauri::{Builder, Runtime};
use tauri_plugin_log::{Target, TargetKind};

pub fn setup<R: Runtime>(mut builder: Builder<R>) -> Builder<R> {
    let logging = tauri_plugin_log::Builder::new()
        .clear_targets()
        .target(Target::new(TargetKind::Stdout))
        .level(log::LevelFilter::Info)
        // .level_for("tauri::plugin", log::LevelFilter::Warn)
        // .level_for("tauri::app", log::LevelFilter::Warn)
        // .level_for("tauri::manager", log::LevelFilter::Warn)
        // .level_for("app::setup", log::LevelFilter::Warn)
        .build();
    builder = builder.plugin(logging);
    builder = builder.plugin(tauri_plugin_opener::init());
    #[cfg(not(any(target_os = "ios", target_os = "android", test)))]
    {
        builder = builder
            .plugin(tauri_plugin_cli::init())
            .plugin(tauri_plugin_fs::init());
    }
    builder // No need for return and semicolon
}
