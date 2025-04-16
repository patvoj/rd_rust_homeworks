use axum::extract::State;
use prometheus::{Counter, Encoder, Gauge, HistogramOpts, HistogramVec, Registry, TextEncoder};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppMetrics {
    pub api_calls_total: Counter,
    pub request_latency_seconds: HistogramVec,
    pub active_connections: Gauge,
    pub registry: Registry,
}

impl AppMetrics {
    pub fn initialize() -> Self {
        let registry = Registry::new();

        let api_calls_total = Counter::new("myapp_api_calls_total", "Total number of API calls")
            .expect("Could not create counter.");

        let histogram_opts = HistogramOpts::new(
            "myapp_http_request_duration_seconds",
            "Histogram of HTTP request latencies in seconds",
        );
        let request_latency_seconds = HistogramVec::new(histogram_opts, &["method", "route"])
            .expect("Could not create histogram");

        let active_connections = Gauge::new(
            "myapp_active_connections",
            "Number of currently active connections",
        )
        .expect("Could not create gauge.");

        registry
            .register(Box::new(api_calls_total.clone()))
            .expect("Failed to register counter");
        registry
            .register(Box::new(request_latency_seconds.clone()))
            .expect("Failed to register histogram");
        registry
            .register(Box::new(active_connections.clone()))
            .expect("Failed to register gauge");

        Self {
            api_calls_total,
            request_latency_seconds,
            active_connections,
            registry,
        }
    }
    pub async fn metrics(State(metrics): State<Arc<Self>>) -> String {
        metrics.gather_metrics()
    }

    pub fn gather_metrics(&self) -> String {
        let encoder = TextEncoder::new();
        let mut buffer = vec![];
        encoder
            .encode(&self.registry.gather(), &mut buffer)
            .unwrap();
        String::from_utf8(buffer).unwrap()
    }
}
