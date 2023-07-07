pub const STACK_BASE: u16 = 0x0100;

/// # Memory Bus
///
/// The memory bus is a way to access memory.
/// In this MOS 6502 emulator, the memory bus is a 16-bit address space, and the data is 8-bit.
pub trait MemoryBus {
    type Data;
    type Addr;
    fn rom(&mut self, data: &[Self::Data]);
    fn write(&mut self, addr: Self::Addr, data: Self::Data);
    fn read(&self, addr: Self::Addr) -> Self::Data;
    fn write_addr(&mut self, addr: Self::Addr, data: Self::Addr);
    fn read_addr(&self, addr: Self::Addr) -> Self::Addr;
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
#[derive(Debug)]
pub struct Memory {
    pub mem: [u8; 0xFFFF],
}

impl Default for Memory {
    fn default() -> Memory {
        Memory { mem: [0; 0xFFFF] }
    }
}

impl std::ops::Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.mem[index as usize]
    }
}

impl std::ops::IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.mem[index as usize]
    }
}

impl MemoryBus for Memory {
    type Data = u8;
    type Addr = u16;

    /// `rom` function loads the program from address `0x8000`.
    fn rom(&mut self, program: &[Self::Data]) {
        self.mem[0x8000..0x8000 + program.len()].copy_from_slice(program);
    }

    /// Write data to memory address
    fn write(&mut self, address: Self::Addr, data: Self::Data) {
        self[address] = data;
    }

    /// Read data from memory address
    fn read(&self, address: Self::Addr) -> Self::Data {
        self[address]
    }

    /// Write 16-bit data to memory address (little endian)
    fn write_addr(&mut self, address: Self::Addr, data: Self::Addr) {
        let [lsb, msb] = data.to_le_bytes();

        self.write(address, lsb);
        self.write(address + 1, msb);
    }

    /// Read 16-bit data from memory address (little endian)
    fn read_addr(&self, address: Self::Addr) -> Self::Addr {
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
        let mut memory = Memory::default();

        memory.write(0x0000, 0x12);
        memory.write(0x0001, 0x34);

        assert_eq!(memory.read(0x0000), 0x12);
        assert_eq!(memory.read(0x0001), 0x34);
    }

    #[test]
    fn test_read_write_addr() {
        let mut memory = Memory::default();

        memory.write_addr(0x0000, 0x1234);

        assert_eq!(memory.read(0x0000), 0x34);
        assert_eq!(memory.read(0x0001), 0x12);
        assert_eq!(memory.read_addr(0x0000), 0x1234);
    }
}
