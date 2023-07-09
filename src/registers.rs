use std::fmt;

/// # Registers
///
/// ## 8 bit
///
/// - `a`: Accumulator Register
/// - `x`: X Index Register
/// - `y`: Y Index Register
/// - `p`: Processor Status Register (`N V - B D I Z C`)
///     - `N`: Negative
///     - `V`: Overflow
///     - `B`: Break
///     - `D`: Decimal
///     - `I`: Interrupt Disable
///     - `Z`: Zero
///     - `C`: Carry
/// - `sp`: Stack Pointer Register
///
/// ## 16 bit
///
/// - `pc`: Program Counter Register
#[derive(Debug, Default)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    pub sp: u8,
    pub pc: u16,
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Registers: A={:02X} X={:02X} Y={:02X} SP={:02X} PC={:04X}",
            self.a, self.x, self.y, self.sp, self.pc
        )?;
        write!(
            f,
            "Flag Registers (NV-B DIZC): {} {} - {}  {} {} {} {}",
            self.get_flag_negative() as u8,
            self.get_flag_overflow() as u8,
            self.get_flag_break() as u8,
            self.get_flag_decimal() as u8,
            self.get_flag_interrupt_disable() as u8,
            self.get_flag_zero() as u8,
            self.get_flag_carry() as u8
        )
    }
}

impl Registers {
    /// Set the flag for the negative bit.
    /// if `value` is `true`, set the negative bit to `1` (`1XXX_XXXX`b).
    pub fn set_flag_negative(&mut self, value: bool) {
        if value {
            self.p |= 0b1000_0000;
        } else {
            self.p &= 0b0111_1111;
        }
    }

    pub fn get_flag_negative(&self) -> bool {
        self.p & 0b1000_0000 != 0
    }

    /// Set the flag for the overflow bit.
    /// if `value` is `true`, set the overflow bit to `1` (`X1XX_XXXX`b).
    pub fn set_flag_overflow(&mut self, value: bool) {
        if value {
            self.p |= 0b0100_0000;
        } else {
            self.p &= 0b1011_1111;
        }
    }

    pub fn get_flag_overflow(&self) -> bool {
        self.p & 0b0100_0000 != 0
    }

    /// Set the flag for the break bit.
    /// if `value` is `true`, set the break bit to `1` (`XXX1_XXXX`b).
    pub fn set_flag_break(&mut self, value: bool) {
        if value {
            self.p |= 0b0001_0000;
        } else {
            self.p &= 0b1110_1111;
        }
    }

    pub fn get_flag_break(&self) -> bool {
        self.p & 0b0001_0000 != 0
    }

    /// Set the flag for the decimal bit.
    /// if `value` is `true`, set the decimal bit to `1` (`XXXX_1XXX`b).
    pub fn set_flag_decimal(&mut self, value: bool) {
        if value {
            self.p |= 0b0000_1000;
        } else {
            self.p &= 0b1111_0111;
        }
    }

    pub fn get_flag_decimal(&self) -> bool {
        self.p & 0b0000_1000 != 0
    }

    /// Set the flag for the interrupt disable bit.
    /// if `value` is `true`, set the interrupt disable bit to `1` (`XXXX_X1XX`b).
    pub fn set_flag_interrupt_disable(&mut self, value: bool) {
        if value {
            self.p |= 0b0000_0100;
        } else {
            self.p &= 0b1111_1011;
        }
    }

    pub fn get_flag_interrupt_disable(&self) -> bool {
        self.p & 0b0000_0100 != 0
    }

    /// Set the flag for the zero bit.
    /// if `value` is `true`, set the zero bit to `1` (`XXXX_XX2X`b).
    pub fn set_flag_zero(&mut self, value: bool) {
        if value {
            self.p |= 0b0000_0010;
        } else {
            self.p &= 0b1111_1101;
        }
    }

    pub fn get_flag_zero(&self) -> bool {
        self.p & 0b0000_0010 != 0
    }

    /// Set the flag for the carry bit.
    /// if `value` is `true`, set the carry bit to `1` (`XXXX_XXXX`b).
    pub fn set_flag_carry(&mut self, value: bool) {
        if value {
            self.p |= 0b0000_0001;
        } else {
            self.p &= 0b1111_1110;
        }
    }

    pub fn get_flag_carry(&self) -> bool {
        self.p & 0b0000_0001 != 0
    }

    pub fn set_zero_negative_flags(&mut self, value: u8) {
        self.set_flag_zero(value == 0);
        self.set_flag_negative(value & 0b1000_0000 != 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_flag() {
        let mut registers = Registers::default();

        registers.set_flag_negative(true);
        assert_eq!(registers.p, 0b1000_0000);

        registers.set_flag_overflow(true);
        assert_eq!(registers.p, 0b1100_0000);

        registers.set_flag_break(true);
        assert_eq!(registers.p, 0b1101_0000);

        registers.set_flag_decimal(true);
        assert_eq!(registers.p, 0b1101_1000);

        registers.set_flag_interrupt_disable(true);
        assert_eq!(registers.p, 0b1101_1100);

        registers.set_flag_zero(true);
        assert_eq!(registers.p, 0b1101_1110);

        registers.set_flag_carry(true);
        assert_eq!(registers.p, 0b1101_1111);
    }
}
