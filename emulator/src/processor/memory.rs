use crate::{Debugger, NoneDebugger};

pub const STACK_BASE: u16 = 0x0100;

/// # Memory Bus
///
/// The memory bus is a way to access memory.
/// In this MOS 6502 emulator, the memory bus is a 16-bit address space, and the data is 8-bit.
pub trait MemoryBus {
    type Data;
    type Addr;
    fn rom(&mut self, data: &[Self::Data]);
    fn reset(&mut self);
    fn write(&mut self, addr: Self::Addr, data: Self::Data);
    fn read(&mut self, addr: Self::Addr) -> Self::Data;
    fn write_addr(&mut self, addr: Self::Addr, data: Self::Addr);
    fn read_addr(&mut self, addr: Self::Addr) -> Self::Addr;
}

/// # Memory Map
///
/// * `0x0000` ~ `0x3FFF`: RAM
///     * `0x0000` ~ `0x00FF`: Zero Page
///     * `0x0100` ~ `0x01FF`: Stack
/// * `0x4000` ~ `0x7FFF`: I/O
/// * `0x8000` ~ `0xFFFF`: ROM
///
/// The actual ROM memory map of the MOS 6502 ranges from `0x8000` - `0xFFF9`, and interrupt vectors are stored in `0xFFFA` - `0xFFFF`.
/// however, since it does not implement interrupts, it is currently not used.
pub struct Memory<T: Debugger> {
    pub mem: [u8; 0xFFFF],
    pub debugger: T,
}

impl<T: Debugger> Memory<T> {
    pub fn new() -> Memory<T> {
        Memory {
            mem: [0; 0xFFFF],
            debugger: T::default(),
        }
    }

    fn debug(&mut self, message: &str) {
        self.debugger.debug(message);
    }
}

impl<T: Debugger> Default for Memory<T> {
    fn default() -> Memory<T> {
        Memory {
            mem: [0; 0xFFFF],
            debugger: T::default(),
        }
    }
}

impl<T: Debugger> std::ops::Index<u16> for Memory<T> {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.mem[index as usize]
    }
}

impl<T: Debugger> std::ops::IndexMut<u16> for Memory<T> {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.mem[index as usize]
    }
}

impl<T: Debugger> MemoryBus for Memory<T> {
    type Data = u8;
    type Addr = u16;

    /// `rom` function loads the program from address `0x8000`.
    fn rom(&mut self, program: &[Self::Data]) {
        self.debug(format!("Load ROM ({} bytes)", program.len()).as_str());
        self.mem[0x8000..0x8000 + program.len()].copy_from_slice(program);
    }

    /// Resets the memory.
    fn reset(&mut self) {
        self.debug("Reset Memory");
        self.mem = [0; 0xFFFF];
    }

    /// Write data to memory address
    fn write(&mut self, address: Self::Addr, data: Self::Data) {
        self.debug(&format!("Write 0x{:04X} = 0x{:02X}", address, data));
        self[address] = data;
    }

    /// Read data from memory address
    fn read(&mut self, address: Self::Addr) -> Self::Data {
        let data = self[address];
        self.debug(&format!("Read 0x{:04X} = 0x{:02X}", address, data));
        data
    }

    /// Write 16-bit data to memory address (little endian)
    fn write_addr(&mut self, address: Self::Addr, data: Self::Addr) {
        self.debug(&format!("Write 0x{:04X} = 0x{:04X}", address, data));
        let [lsb, msb] = data.to_le_bytes();

        self.write(address, lsb);
        self.write(address + 1, msb);
    }

    /// Read 16-bit data from memory address (little endian)
    fn read_addr(&mut self, address: Self::Addr) -> Self::Addr {
        self.debug(&format!("Read 0x{:04X}", address));
        let lsb = self.read(address);
        let msb = self.read(address + 1);

        u16::from_le_bytes([lsb, msb])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let mut memory = Memory::<NoneDebugger>::default();

        memory.write(0x0000, 0x12);
        memory.write(0x0001, 0x34);

        assert_eq!(memory.read(0x0000), 0x12);
        assert_eq!(memory.read(0x0001), 0x34);
    }

    #[test]
    fn test_read_write_addr() {
        let mut memory = Memory::<NoneDebugger>::default();

        memory.write_addr(0x0000, 0x1234);

        assert_eq!(memory.read(0x0000), 0x34);
        assert_eq!(memory.read(0x0001), 0x12);
        assert_eq!(memory.read_addr(0x0000), 0x1234);
    }
}

/// | 0x0000 | 00 00 .. 00 00 | ................ |
pub type MemoryDumpResult = Vec<(u16, [u8; 16], [char; 16])>;

pub fn memory_hexdump(memory: [u8; 0xFFFF], start: u16, end: u16) -> MemoryDumpResult {
    let mut memory: Memory<NoneDebugger> = Memory {
        mem: memory,
        ..Default::default()
    };
    let mut result = Vec::new();

    for addr in (start..=end).step_by(16) {
        let mut line = ([0; 16], [' '; 16]);

        for i in 0..16 {
            if addr + i > u16::MAX - 1 {
                break;
            }
            let data = memory.read(addr + i);

            line.0[i as usize] = data;

            if data.is_ascii_control() {
                line.1[i as usize] = '.';
            } else {
                line.1[i as usize] = data as char;
            }
        }

        result.push((addr, line.0, line.1));
    }

    result
}

pub fn memory_hexdump_string(memory: [u8; 0xFFFF], start: u16, end: u16) -> String {
    let mut memory: Memory<NoneDebugger> = Memory {
        mem: memory,
        ..Default::default()
    };
    let mut result = Vec::new();

    for addr in (start..=end).step_by(16) {
        let mut line = format!("[0x{:04X}] ", addr);

        for i in 0..16 {
            if addr + i > u16::MAX - 1 {
                line.push_str("   ");
                break;
            }
            let data = memory.read(addr + i);
            line.push_str(&format!("{:02X} ", data));
        }

        line.push_str("| ");

        for i in 0..16 {
            if addr + i > u16::MAX - 1 {
                line.push(' ');
                break;
            }
            let data = memory.read(addr + i);
            if data.is_ascii_control() {
                line.push('.');
            } else {
                line.push(data as char);
            }
        }

        line.push_str(" |");

        result.push(line);
    }

    result.join("\n")
}
