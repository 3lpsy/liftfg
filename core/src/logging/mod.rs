use anyhow::{Error, Result};
use std::env;
use std::path::PathBuf;
use tracing::level_filters::LevelFilter;
use tracing::{debug, info, warn, Subscriber};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::Layered;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::reload::Handle;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};
use tracing_subscriber::{reload, Layer, Registry};

pub type LogHandle = Handle<
    Vec<Box<dyn Layer<Layered<EnvFilter, Registry>> + Send + Sync>>,
    Layered<EnvFilter, Registry>,
>;

pub type LayersHandle = Handle<
    Vec<Box<dyn Layer<Layered<reload::Layer<EnvFilter, Registry>, Registry>> + Send + Sync>>,
    Layered<reload::Layer<EnvFilter, Registry>, Registry>,
>;

pub type FilterHandle = Handle<EnvFilter, Registry>;

pub fn init() -> Result<(LayersHandle, FilterHandle)> {
    let (layers, layers_handle) = reload::Layer::new(vec![console_layer()?]);
    let (filter_layer, filter_handle) = reload::Layer::new(env_filter());
    let subscriber = tracing_subscriber::registry()
        .with(filter_layer)
        .with(layers);

    match tracing::subscriber::set_global_default(subscriber) {
        Ok(_) => {
            debug!("Default logging layer initialized...");
            Ok((layers_handle, filter_handle))
        }
        Err(_e) => Err(Error::msg("Tracing subscriber already registered.")),
    }
}
pub fn reload_filter(handle: FilterHandle) -> Result<()> {
    // This will hange env_filter() is called in modify
    let f = env_filter();
    handle.modify(move |filter| {
        *filter = f;
    })?;
    debug!("Filter reloaded...");
    Ok(())
}
pub fn setup_fs(log_dir: &PathBuf, handle: LayersHandle) -> Result<()> {
    handle.modify(|filter| {
        (*filter).push(fs_layer(log_dir).unwrap());
    })?;
    debug!("Filesyste logging initialized...");
    Ok(())
}

fn env_filter() -> EnvFilter {
    let f = match EnvFilter::try_from_default_env() {
        Ok(f) => {
            info!(
                "RUST_LOG: {:?}",
                env::var("RUST_LOG").unwrap_or("".to_owned())
            );
            f
        }
        Err(e) => {
            warn!(
                "Creating default info filter. Could not parse RUST_LOG: {:?}",
                e
            );
            EnvFilter::default().add_directive(LevelFilter::INFO.into())
        }
    };
    // f.add_directive("app::emit=trace".parse().unwrap());
    f
}

fn console_layer<S>() -> Result<Box<dyn Layer<S> + Send + Sync + 'static>>
where
    S: Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    // let timer = UtcTime::rfc_3339();
    Ok(fmt::layer()
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
