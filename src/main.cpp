#include "ffi_otel/src/lib.rs.h"
#include "rust/cxx.h"
#include <string>
#include <iostream>

int main()
{
    auto my_layer = MyLayer::create();
    my_layer->trace_error();
    // create();
    // trace_error();
    
    return 0;
}

















































































// auto my_layer = MyLayer::create();
// my_layer->trace_error();




// // #include "ffi_otel/src/lib.rs"
// // #include <iostream>

// // int main()
// // {
// //     GetTracerProvider();
// //     return 0;
// // }

// // #include "rust/cxx.h"
// #include "ffi_otel/src/lib.rs"
// // #include "opentelemetry/trace/tracer_provider.h"
// // #include "opentelemetry/trace/tracer.h"

// #include <iostream>
// #include "opentelemetry/sdk/version/version.h"
// #include "opentelemetry/trace/provider.h"

// int main() {
//     auto rust_tracer_provider = opentelemetry::trace::Provider::GetTracerProvider();
//     auto rust_tracer = rust_tracer_provider->GetTracer("foo_library", "1.0.0");

//     TracerProvider* tracer_provider = get_tracer_provider();
//     // Use tracer_provider

//     // Remember to delete tracer_provider when you're done using it
//     // delete tracer_provider;

//     return 0;
// }