#[allow(dead_code)]
mod bindings;

#[cfg(target_arch = "wasm32")]
use bindings::wasi::logging::logging::{log, Level};
use bindings::Guest;

struct Component;

impl Guest for Component {
    fn add(a: f64, b: f64) -> f64 {
        let result = a + b;

        #[cfg(target_arch = "wasm32")]
        log(
            Level::Info,
            "guest:rust:add",
            format!("{a} + {b} = {result}").as_str(),
        );

        result
    }

    fn divide(a: f64, b: f64) -> f64 {
        if b == 0.0 {
            #[cfg(target_arch = "wasm32")]
            log(
                Level::Error,
                "guest:rust:divide",
                format!("Division by zero error").as_str(),
            );

            panic!()
        }

        let result = a / b;

        #[cfg(target_arch = "wasm32")]
        log(
            Level::Info,
            "guest:rust:divide",
            format!("{a} / {b} = {result}").as_str(),
        );

        result
    }
}
