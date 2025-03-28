use anyhow::{Error, Result};
use std::env;
use std::path::PathBuf;
use tracing::level_filters::LevelFilter;
use tracing::{debug, warn, Subscriber};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::Layered;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::reload::Handle;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};
use tracing_subscriber::{reload, Layer, Registry};
pub type LayersHandle = Handle<
    Vec<Box<dyn Layer<Layered<reload::Layer<EnvFilter, Registry>, Registry>> + Send + Sync>>,
    Layered<reload::Layer<EnvFilter, Registry>, Registry>,
>;

pub type FilterHandle = Handle<EnvFilter, Registry>;

#[cfg(debug_assertions)]
static DEFAULT_TRACING_LEVEL_FILTER: LevelFilter = LevelFilter::INFO;
#[cfg(not(debug_assertions))]
static DEFAULT_LOG_LEVEL_FILTER: log::LevelFilter = log::LevelFilter::Warn;
#[cfg(not(debug_assertions))]
static DEFAULT_TRACING_LEVEL_FILTER: LevelFilter = LevelFilter::WARN;

// ISSUES
// without LogTracer in IOS, log messages will not be sent to trace layers
// Can use OsLogger to at least get logs output
//
// In Prod, logs should be sent to files anyways so console logging should be disabled
pub fn init() -> Result<(LayersHandle, FilterHandle)> {
    let (layers, layers_handle) = reload::Layer::new(vec![console_layer()?]);

    let mut bad_filter = false;
    let filter = match EnvFilter::try_from_default_env() {
        Ok(f) => f,
        Err(_e) => {
            bad_filter = true;
            EnvFilter::default().add_directive(DEFAULT_TRACING_LEVEL_FILTER.into())
        }
    };
    let (filter_layer, filter_handle) = reload::Layer::new(filter);
    let subscriber = tracing_subscriber::registry()
        .with(filter_layer)
        .with(layers);

    // output layer
    // probably only useful for dev
    #[cfg(target_os = "ios")]
    let ios_tracer_layer = tracing_oslog::OsLogger::new("org.liftfg.app", "default");
    #[cfg(target_os = "ios")]
    let subscriber = subscriber.with(ios_tracer_layer);

    let registry = match tracing::subscriber::set_global_default(subscriber) {
        Ok(_) => {
            // unrecoverable on ios sijmulator

            // #[cfg(not(target_os = "ios"))]
            // {
            //     if let Err(e) = tracing_log::LogTracer::init() {
            //         warn!("Log tracer connection failed: {:?}", e);
            //     } else {
            //         debug!("Log tracer connected... ");
            //     }
            // }

            // Will not capture frontend errors in prod
            // Need to still adapt tracing
            // #[cfg(all(target_os = "ios", debug_assertions))]
            // {
            //     use oslog::OsLogger;
            //     OsLogger::new("org.liftfg.app")
            //         .level_filter(DEFAULT_LOG_LEVEL_FILTER)
            //         .category_level_filter("Settings", log::LevelFilter::Trace)
            //         .init()
            //         .unwrap();
            // }
            Ok((layers_handle, filter_handle))
        }
        Err(_e) => Err(Error::msg("Tracing subscriber already registered.")),
    };
    if bad_filter {
        warn!(
            "Falling back to 'info' RUST_LOG. RUST_LOG filter could not be parsed: {:?}",
            env::var("RUST_LOG").unwrap_or("".to_owned())
        )
    } else {
        debug!(
            "RUST_LOG: {:?}",
            env::var("RUST_LOG").unwrap_or("".to_owned())
        );
    }
    registry
}
pub fn reload_filter(handle: FilterHandle) -> Result<()> {
    // This will hange if called in callback
    let mut bad_filter = false;
    let f = match EnvFilter::try_from_default_env() {
        Ok(f) => f,
        Err(_e) => {
            bad_filter = true;
            EnvFilter::default().add_directive(LevelFilter::INFO.into())
        }
    };
    handle.modify(move |filter| {
        *filter = f;
    })?;
    if bad_filter {
        warn!(
            "Falling back to 'info' RUST_LOG. RUST_LOG filter could not be parsed: {:?}",
            env::var("RUST_LOG").unwrap_or("".to_owned())
        )
    } else {
        debug!("Filter reloaded...");
    }
    Ok(())
}
pub fn setup_fs(log_dir: &PathBuf, handle: LayersHandle) -> Result<()> {
    handle.modify(|filter| {
        (*filter).push(fs_layer(log_dir).unwrap());
    })?;
    debug!("Filesystem logging initialized...");
    Ok(())
}

fn console_layer<S>() -> Result<Box<dyn Layer<S> + Send + Sync + 'static>>
where
    S: Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    let layer = fmt::layer();
    #[cfg(target_os = "ios")]
    let layer = layer.with_ansi(false); // no color in console on ios
    Ok(layer
        // .with_timer(timer)
        .with_thread_ids(true)
        .with_target(true)
        .compact()
        .boxed())
}
// currently broken, need filter id
pub fn fs_layer<S>(log_dir: &PathBuf) -> Result<Box<dyn Layer<S> + Send + Sync + 'static>>
where
    S: Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir)?;
    }
    let file_appender = RollingFileAppender::new(Rotation::DAILY, log_dir, "app.log");
    let timer = UtcTime::rfc_3339();

    Ok(fmt::layer()
        .with_writer(file_appender)
        .with_timer(timer.clone())
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(false)
        .boxed())
}
