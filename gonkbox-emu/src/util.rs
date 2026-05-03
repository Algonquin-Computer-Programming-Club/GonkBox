pub mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    unsafe extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }
}

macro_rules! log {
    ($($t:tt)*) => (
        if cfg!(target_arch = "wasm32") {
            $crate::util::js::log(&format_args!($($t)*).to_string())
        } else {
            println!($($t)*);
        }
    )
}

pub(super) use log;
