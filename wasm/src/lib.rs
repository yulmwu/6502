use assembler::Assembler;
use emulator::{
    cpu::Cpu,
    memory::{memory_hexdump_string, Memory},
    Cpu6502, CpuDebugger, DebugKind, Debugger,
};
use js_sys::Function;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

type DebugCallBack = Box<dyn Fn(&str)>;

#[wasm_bindgen]
pub struct Emulator {
    cpu: Cpu6502<WasmDebugger>,
}

// #[wasm_bindgen]
#[derive(Default)]
struct WasmDebugger {
    debug_callback: Option<DebugCallBack>,
}

impl Debugger for WasmDebugger {
    fn debug(&mut self, message: &str, _: DebugKind) {
        if let Some(debug_callback) = &self.debug_callback {
            debug_callback(message);
        }
    }
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
        let mut cpu = Cpu6502::<WasmDebugger>::new(Memory::new());
        cpu.debugger = WasmDebugger::default();

        Self {
            cpu: Cpu::default(),
        }
    }

    pub fn set_cpu_debug_callback(&mut self, debug_callback: Function) {
        self.cpu.debugger.debug_callback = Some(Box::new(move |msg| {
            debug_callback
                .call1(&JsValue::NULL, &JsValue::from_str(msg))
                .unwrap();
        }));
    }

    pub fn set_memory_debug_callback(&mut self, debug_callback: Function) {
        self.cpu.memory.debugger.debug_callback = Some(Box::new(move |msg| {
            debug_callback
                .call1(&JsValue::NULL, &JsValue::from_str(msg))
                .unwrap();
        }));
    }

    pub fn set_registers_debug_callback(&mut self, debug_callback: Function) {
        self.cpu.registers.debugger.debug_callback = Some(Box::new(move |msg| {
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
        memory_hexdump_string(self.cpu.memory.mem, start, end)
    }

    pub fn cpu_status(&self) -> String {
        format!("{}", self.cpu)
    }

    pub fn assemble(&self, source: &str) -> AssemblerResult {
        let src = Assembler::new(source).assemble();

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
