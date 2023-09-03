pub trait Debugger {
    fn step(&mut self) -> u8;
    fn debug(&self, message: &str);
}
