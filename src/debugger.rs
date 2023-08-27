use std::time::Duration;

pub trait Debugger {
    fn set_interval(&mut self, duration: Option<Duration>);
    fn step(&mut self);
    fn debug(&self, message: &str);
}
