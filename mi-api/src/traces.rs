use opentelemetry_otlp::WithExportConfig;
use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub fn init_tracer() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy()
        .add_directive("tower_http=info".parse().unwrap())
        .add_directive("sqlx::query=debug".parse().unwrap());

    let honeycomb_key = std::env::var("HONEYCOMB_KEY").expect("HONEYCOMB_KEY is not set");

    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint("https://api.honeycomb.io/v1/traces")
        .with_headers(
            [("x-honeycomb-team".to_string(), honeycomb_key)]
                .into_iter()
                .collect(),
        )
        .with_timeout(std::time::Duration::from_secs(5));

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("failed to install otlp tracer");

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
