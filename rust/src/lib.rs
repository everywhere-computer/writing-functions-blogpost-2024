#[allow(dead_code)]
mod bindings;

#[cfg(target_arch = "wasm32")]
use bindings::wasi::logging::logging::{log, Level};
use bindings::Guest;

struct Component;

impl Guest for Component {
    fn add(a: i32, b: i32) -> i32 {
        let result = a + b;

        #[cfg(target_arch = "wasm32")]
        log(
            Level::Info,
            "guest:add",
            format!("{a} + {b} = {result}").as_str(),
        );

        result
    }
}
