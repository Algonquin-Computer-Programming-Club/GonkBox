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
        $crate::util::js::log(&format_args!($($t)*).to_string())
    )
}

pub(super) use log;

pub fn u16_to_bytes(input: &u16) -> [u8; 2] {
    [(input % 256) as u8, (input >> 8) as u8]
}

pub fn bytes_to_u16(input: &[u8; 2]) -> u16 {
    input[0] as u16 + ((input[1] as u16) << 8)
}
