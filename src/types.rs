// Labeled V0-VF -> 0 -> F, 0 -> 15
pub const VF: usize = 15;
pub type GeneralRegisters = [u8; 16];
pub type AddressRegister = u16;
pub type Stack = Vec<u16>;
pub type ProgramCounter = u16;