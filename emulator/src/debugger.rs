pub trait Debugger {
    fn step(&mut self);
    fn debug(&self, message: &str);
}
