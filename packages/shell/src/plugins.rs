use tauri::{Builder, Runtime};
// use tauri_plugin_log::{Target, TargetKind};
#[cfg(all(debug_assertions, feature = "devtools"))]
use tauri_plugin_devtools;
#[cfg(all(debug_assertions, feature = "devtools"))]
use tauri_plugin_devtools_app;
use tracing::debug;

pub fn load<R: Runtime>(mut builder: Builder<R>) -> Builder<R> {
    let logging = tauri_plugin_log::Builder::new().skip_logger().build();
    debug!("Loading plugins...");
    builder = builder.plugin(logging);
    builder = builder.plugin(tauri_plugin_opener::init());
    builder = builder.plugin(tauri_plugin_fs::init());
    // CrabNebula DevTools
    #[cfg(all(debug_assertions, feature = "devtools"))]
    {
        builder = builder.plugin(tauri_plugin_devtools::init());
    }

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        builder = builder.plugin(tauri_plugin_cli::init());
    }
    builder
}
