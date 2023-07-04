/// Memory Allocation
///
/// * `0x0000` ~ `0x3FFF`: RAM
/// * `0x4000` ~ `0x7FFF`: I/O
/// * `0x8000` ~ `0xFFFF`: ROM
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

impl Memory {
    /// `rom` function loads the program from address `0x8000`.
    pub fn rom(&mut self, program: &[u8]) {
        self.mem[0x8000..0x8000 + program.len()].copy_from_slice(program);
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self[address] = data;
    }

    pub fn read(&self, address: u16) -> u8 {
        self[address]
    }

    pub fn write_16(&mut self, address: u16, data: u16) {
        // 0x1234 - low: 0x34, high: 0x12
        let low = (data & 0x00FF) as u8;
        let high = ((data & 0xFF00) >> 8) as u8;

        self.write(address, low);
        self.write(address + 1, high);
    }

    pub fn read_16(&self, address: u16) -> u16 {
        let low = self.read(address);
        let high = self.read(address + 1);

        u16::from_le_bytes([low, high])
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
    fn test_read_write_16() {
        let mut memory = Memory::default();

        memory.write_16(0x0000, 0x1234);

        assert_eq!(memory.read_16(0x0000), 0x1234);
    }
}
