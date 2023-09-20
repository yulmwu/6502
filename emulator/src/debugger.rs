#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DebugKind {
    Info,
    Warn,
    Error,
}

pub trait Debugger: Default {
    fn debug(&mut self, message: &str, kind: DebugKind);
}

pub trait CpuDebugger {
    fn step(&mut self) -> u8;
}

#[derive(Default)]
pub struct NoneDebugger;

impl Debugger for NoneDebugger {
    fn debug(&mut self, _: &str, _: DebugKind) {}
}
