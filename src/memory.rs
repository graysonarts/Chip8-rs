// pub type Memory = [u8; 4096];

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct Memory([u8; 4096], usize);
impl Memory {
	pub fn from_file(f: PathBuf) -> std::io::Result<Self> {
		let mut memory = Memory([0; 4096], 0);
		let mut rom = File::open(f)?;
		let size = rom.read(&mut memory.0)?;
		memory.1 = size;

		Ok(memory)
	}
}

impl std::fmt::Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			let result : String = self.0[..self.1].chunks(16)
				.map(|e| {
					let result : String = e.iter().map(|b| format!("{:02x} ", b)).collect();
					format!("{}\n", result)
				})
				.collect();
			write!(f, "{}", result)
    }

}