use std::time::Duration;
use opentelemetry::sdk::export::trace::stdout;
use opentelemetry_otlp::WithExportConfig;
use tracing::{error, info, span, instrument};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, Registry, registry};
use opentelemetry::sdk::Resource;
use opentelemetry::trace::TraceError;
use opentelemetry::{global, sdk::trace as sdktrace};
use opentelemetry::{trace::Tracer};


fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
        .with_trace_config(
            sdktrace::config().with_resource(Resource::default()),
        )
        .install_batch(opentelemetry::runtime::Tokio)
}

#[instrument]
fn foo() {
    // let trace = register_dist_tracing_root(TraceId::default(), None);
    // println!("trace value: {:?}", trace);
    info!("test");
}

#[tokio::main]
async fn main() {
    let tracer = init_tracer().unwrap();
    let telemetry = tracing_opentelemetry::layer()
        .with_tracer(tracer);

    let subscriber = Registry::default().with(telemetry);

    tracing::subscriber::set_global_default(subscriber).expect("setting global default failed");
    foo();
    tokio::time::sleep(Duration::from_secs(10)).await;
}
