use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry::trace::{Tracer, TracerProvider as _};
use tracing::{debug, error, info, span, Level};
use tracing_subscriber::{self, fmt, layer::SubscriberExt, Registry};
use tracing_opentelemetry::OpenTelemetryLayer;
use opentelemetry_semantic_conventions::{
    attribute::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};
use opentelemetry::KeyValue;
use opentelemetry_sdk::Resource;


fn init_tracing() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .with_endpoint("http://172.24.0.3:4318/v1/traces")
        .with_timeout(std::time::Duration::from_secs(5))
        .build()?;

    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(otlp_exporter)
        .with_resource(Resource::builder_empty().with_service_name("new-agent").build())
        .build();

    let tracer = provider.tracer("app_trace");

    let telemetry_layer = OpenTelemetryLayer::new(tracer);

    let env_filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing::Level::DEBUG.into())
        .from_env_lossy();

    let subscriber = Registry::default()
        .with(telemetry_layer)
        .with(env_filter)
        .with(fmt::layer());

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    
    init_tracing()?;

    let root_span = span!(Level::DEBUG, "main_execution", work_units = 2);
    let _enter = root_span.enter();

    info!("Starting main execution");

    Ok(())
}