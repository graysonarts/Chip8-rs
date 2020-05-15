use crate::errors::RuntimePanic;
use crate::memory::Memory;
use crate::types::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum OpCode {
	// 2NNN
	Call(u16),
	// 4XNN
	NotEqual(usize, u8),
	// 6XNN
	SetRegister(usize, u8),

	// ANNN
	SetAddr(u16),

	Unknown(u16)
}

impl From<u16> for OpCode {
	fn from(value: u16) -> OpCode {
		let [hb, lb] = value.to_be_bytes();
		let category = hb >> 4;
		match category {
			0x02 => OpCode::Call(u16::from_be_bytes([hb & 0x0F, lb])),
			0x04 => OpCode::NotEqual((hb & 0x0f).into(), lb),
			0x06 => OpCode::SetRegister((hb & 0x0F).into(), lb),
			0x0A => OpCode::SetAddr(u16::from_be_bytes([hb & 0x0F, lb])),
			_ => OpCode::Unknown(value)
		}
	}
}

impl OpCode {
	pub fn execute(&self, pc: &mut u16, memory: &mut Memory, registers: &mut GeneralRegisters, i: &mut AddressRegister, stack: &mut Stack) -> Result<(), RuntimePanic> {
		trace!("{:?}", self);
		use OpCode::*;
		match self {
			Call(addr) => {
				let offset_addr = *addr - 0x200; // Originally CHIP-8 Interpretter took up the first 512 bytes of memory.
				debug!("{:04x}", offset_addr);
				stack.push(*pc);
				*pc = offset_addr;
			},
			NotEqual(r, v) => {
				if registers[*r] != *v {
					*pc += 2;
				}
			},
			SetRegister(r, v) => {
				registers[*r] = *v;
			},
			SetAddr(addr) => *i = *addr - 0x200,
			_ => return Err(RuntimePanic::UnknownOpcode(*self))
		};

		Ok(())
	}
}

 #[cfg(test)]
 mod test_super {
	 #![allow(non_snake_case)]
		 use super::*;

		 #[test]
		 fn test_6XNN_against_V0() {
			 let actual = 0x6000u16.into();
			 assert_eq!(OpCode::SetRegister(0, 0), actual);
		 }

		 #[test]
		 fn test_6XNN_against_VF() {
			 let actual = 0x6FFFu16.into();
			 assert_eq!(OpCode::SetRegister(0xF, 0xFF), actual);
		 }
 }