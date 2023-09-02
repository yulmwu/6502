use assembler::Assembler;
use emulator::{
    cpu::Cpu,
    memory::{memory_hexdump, Memory},
    Debugger,
};
use js_sys::Function;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

#[wasm_bindgen]
pub struct Emulator {
    cpu: Cpu<Memory>,
}

#[wasm_bindgen]
#[derive(Clone)]
pub enum AssemblerResultKind {
    Ok,
    Err,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct AssemblerResult {
    kind: AssemblerResultKind,
    value: Option<Vec<u8>>,
    error: Option<String>,
}

#[wasm_bindgen]
impl AssemblerResult {
    pub fn kind(&self) -> AssemblerResultKind {
        self.kind.clone()
    }

    pub fn value(&self) -> Option<Vec<u8>> {
        self.value.clone()
    }

    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            cpu: Cpu::default(),
        }
    }

    pub fn set_cpu_debug_callback(&mut self, debug_callback: Function) {
        self.cpu.set_debug_callback(Box::new(move |msg| {
            debug_callback
                .call1(&JsValue::NULL, &JsValue::from_str(msg))
                .unwrap();
        }));
    }

    pub fn set_memory_debug_callback(&mut self, debug_callback: Function) {
        self.cpu.memory.set_debug_callback(Box::new(move |msg| {
            debug_callback
                .call1(&JsValue::NULL, &JsValue::from_str(msg))
                .unwrap();
        }));
    }

    pub fn set_registers_debug_callback(&mut self, debug_callback: Function) {
        self.cpu.registers.set_debug_callback(Box::new(move |msg| {
            debug_callback
                .call1(&JsValue::NULL, &JsValue::from_str(msg))
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

    pub fn memory_hexdump(&self, start: u16, end: u16) -> String {
        memory_hexdump(self.cpu.memory.mem, start, end)
    }

    pub fn cpu_status(&self) -> String {
        format!("{}", self.cpu)
    }

    pub fn assemble(&self, source: &str) -> AssemblerResult {
        let src = Assembler::new(source.to_string()).assemble();

        match src {
            Ok(src) => AssemblerResult {
                kind: AssemblerResultKind::Ok,
                value: Some(src),
                error: None,
            },
            Err(err) => AssemblerResult {
                kind: AssemblerResultKind::Err,
                value: None,
                error: Some(err.to_string()),
            },
        }
    }
}
