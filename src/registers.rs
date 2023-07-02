/// # Registers
///
/// ## 8 bit
///
/// - `a`: Accumulator Register
/// - `x`: X Index Register
/// - `y`: Y Index Register
/// - `p`: Processor Status Register (`N V B D I Z C`)
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
