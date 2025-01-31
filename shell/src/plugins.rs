use tauri::{Builder, Runtime};
// use tauri_plugin_log::{Target, TargetKind};
use tracing::debug;

pub fn load<R: Runtime>(mut builder: Builder<R>) -> Builder<R> {
    // let logging = tauri_plugin_log::Builder::new()
    //     .clear_targets()
    //     .target(Target::new(TargetKind::Stdout))
    //     .level(log::LevelFilter::Info)
    //     .build();
    let logging = tauri_plugin_log::Builder::new().skip_logger().build();
    debug!("Loading plugins...");
    builder = builder.plugin(logging);
    builder = builder.plugin(tauri_plugin_opener::init());
    builder = builder.plugin(tauri_plugin_fs::init());
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        builder = builder.plugin(tauri_plugin_cli::init());
    }
    builder // No need for return and semicolon
}
