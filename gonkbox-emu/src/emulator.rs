#![allow(unused)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GonkBoxEmu {
    memory: Vec<u8>,

    // gp registers
    bill: u16,
    charlie: u16,
    tim: u16,

    // special registers
    paul: u16,
    microwave: u16,
}

#[wasm_bindgen]
impl GonkBoxEmu {
    #[wasm_bindgen(constructor)]
    pub fn new(size: u16) -> GonkBoxEmu {
        GonkBoxEmu {
            memory: vec![0; size.into()],
            bill: 0,
            charlie: 0,
            tim: 0,
            paul: 0,
            microwave: 0,
        }
    }

    // pub fn get_memory(&self) -> Vec<u8> {
    //     self.memory
    // }

    #[wasm_bindgen(js_name = "resetMemory")]
    pub fn reset_memory(&mut self) {
        self.memory = vec![0; self.memory.capacity()]
    }

    #[wasm_bindgen(js_name = "resetMemorySized")]
    pub fn reset_memory_sized(&mut self, capacity: u16) {
        self.memory = vec![0; capacity.into()];
    }

    #[wasm_bindgen(js_name = "step")]
    pub fn step(&mut self) {}
}
