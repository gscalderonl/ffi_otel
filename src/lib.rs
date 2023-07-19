// use opentelemetry_api::KeyValue;
// use opentelemetry_appender_tracing::layer;
// use opentelemetry_sdk::{
//     logs::{Config, LoggerProvider},
//     Resource,
// };
// use tracing::error;
// use tracing_subscriber::prelude::*;

// #[cxx::bridge]

// mod ffi
// {
//     extern "Rust"
//     {
//         fn init_tracing();
//         fn trace_error();
//     }
// }

// pub fn init_tracing()
// {
//     let exporter = opentelemetry_stdout::LogExporter::default();
//     let provider: LoggerProvider = LoggerProvider::builder()
//         .with_config(
//             Config::default().with_resource(Resource::new(vec![KeyValue::new(
//                 "service.name",
//                 "log-appender-tracing-example",
//             )])),
//         )
//         .with_simple_exporter(exporter)
//         .build();
//     let layer = layer::OpenTelemetryTracingBridge::new(&provider);
//     tracing_subscriber::registry().with(layer).init();
//     error!(target: "my-system", event_id = 20, event_name = "my-event_name", user_name = "otel", user_email = "otel@opentelemetry.io");
//     drop(provider);
// }

// pub fn trace_error()
// {
//     init_tracing();
//     error!(target: "my-system", event_id = 20, event_name = "my-event_name", user_name = "otel", user_email = "otel@opentelemetry.io");
// }



use opentelemetry_api::KeyValue;
use opentelemetry_appender_tracing::layer;
use opentelemetry_sdk::{
    logs::{Config, LoggerProvider},
    Resource,
};
use tracing::error;
use tracing_subscriber::prelude::*;

#[cxx::bridge]

mod ffi {
    extern "Rust"
    {
        type MyLayer;
    }
}
// trait MyTrace {
//     fn trace_error(&self);
// }

struct MyLayer{
    provider: LoggerProvider,
}

impl MyLayer {
    fn new() -> MyLayer {
        let exporter = opentelemetry_stdout::LogExporter::default();
         let provider = LoggerProvider::builder()
            .with_config(
                Config::default().with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "log-appender-tracing-example",
                )])),
            )
            .with_simple_exporter(exporter)
            .build();
        
        MyLayer {    
            provider
        }
    }

    fn trace_error(&self) {
        let layer = layer::OpenTelemetryTracingBridge::new(&self.provider);
        tracing_subscriber::registry().with(layer).init();
        error!(target: "my-system", event_id = 20, event_name = "my-event_name", user_name = "otel", user_email = "test");
    }
}

// impl MyTrace for MyLayer {

//     fn trace_error(&self) {
//         let layer = layer::OpenTelemetryTracingBridge::new(&self.provider);
//         tracing_subscriber::registry().with(layer).init();
//         error!(target: "my-system", event_id = 20, event_name = "my-event_name", user_name = "otel", user_email = "test");
//     }
// }