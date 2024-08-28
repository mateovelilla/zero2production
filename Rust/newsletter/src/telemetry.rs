use tracing_subscriber::fmt::MakeWriter;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;
use tracing::Subscriber;
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink
    ) -> impl Subscriber + Sync + Send 
    where
        Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let formmating_layer = BunyanFormattingLayer::new(
        name,
        sink
        );
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formmating_layer)
}
pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    LogTracer::init().expect("Failed to set subscriber");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
