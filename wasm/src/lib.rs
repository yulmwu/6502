use emulator::{
    cpu::Cpu,
    memory::{memory_hexdump, Memory},
    Assembler, Debugger,
};
use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Emulator {
    cpu: Cpu<Memory>,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            cpu: Cpu::default(),
        }
    }

    pub fn set_cpu_debug_callback(&mut self, debug_callback: Function) {
        self.cpu.set_debug_callback(Box::new(move |msg| {
            debug_callback
                .call1(&JsValue::NULL, &JsValue::from_str(&msg))
                .unwrap();
        }));
    }

    pub fn set_memory_debug_callback(&mut self, debug_callback: Function) {
        self.cpu.memory.set_debug_callback(Box::new(move |msg| {
            debug_callback
                .call1(&JsValue::NULL, &JsValue::from_str(&msg))
                .unwrap();
        }));
    }

    pub fn load(&mut self, data: Vec<u8>) {
        self.cpu.load(&data);
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn execute(&mut self) {
        self.cpu.execute();
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }

    pub fn memory_hexdump(&mut self, start: u16, end: u16) -> String {
        memory_hexdump(&mut self.cpu.memory, start, end)
    }

    pub fn cpu_status(&self) -> String {
        format!("{}", self.cpu)
    }

    pub fn assemble(&self, source: &str) -> Vec<u8> {
        Assembler::new(source.to_string()).assemble()
    }
}
