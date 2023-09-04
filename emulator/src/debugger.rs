pub trait Debugger: Default {
    fn debug(&mut self, message: &str);
}

pub trait CpuDebugger {
    fn step(&mut self) -> u8;
}

#[derive(Default)]
pub struct NoneDebugger;

impl Debugger for NoneDebugger {
    fn debug(&mut self, _: &str) {}
}
