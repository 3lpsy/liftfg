use anyhow::{anyhow, Error, Result};
use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::level_filters::LevelFilter;
use tracing::{Level, Subscriber};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::filter::Directive;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::reload::Handle;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};
use tracing_subscriber::{reload, Layer, Registry};

pub type LogHandle = Handle<Vec<Box<dyn Layer<Registry> + Send + Sync>>, Registry>;

pub fn setup() -> Result<LogHandle> {
    let layers = default_layers()?;
    let (layers, reload_handle) = reload::Layer::new(layers);
    let subscriber = tracing_subscriber::registry().with(layers);
    match tracing::subscriber::set_global_default(subscriber) {
        Ok(_) => Ok(reload_handle),
        Err(_e) => Err(Error::msg("Tracing subscriber already registered.")),
    }
}
pub fn setup_fs(log_dir: &PathBuf, handle: LogHandle) -> Result<()> {
    let mut layers = default_layers()?;
    layers.push(fs_layer(log_dir)?);
    handle.modify(|filter| *filter = layers)?;
    Ok(())
}

fn default_layers<S>() -> Result<Vec<Box<dyn Layer<S> + Send + Sync + 'static>>>
where
    S: Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    let env_layer = EnvFilter::try_from_default_env()
        .unwrap_or(EnvFilter::default().add_directive(LevelFilter::ERROR.into()))
        .boxed();
    let console_error_ = console_layer()?;
    Ok(vec![env_layer, console_error_])
}

fn console_layer<S>() -> Result<Box<dyn Layer<S> + Send + Sync + 'static>>
where
    S: Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    let timer = UtcTime::rfc_3339();
    Ok(fmt::layer()
        .with_timer(timer)
        .with_thread_ids(true)
        .with_target(true)
        .boxed())
}
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
