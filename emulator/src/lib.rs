pub mod debugger;
pub mod processor;

pub use debugger::*;
pub use processor::*;

use processor::{cpu::Cpu, memory::Memory};

pub type Cpu6502<D> = Cpu<Memory<D>, D, D>;
