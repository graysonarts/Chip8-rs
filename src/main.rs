#![feature(no_more_cas)]

use std::fs::File;
use std::path::PathBuf;
use std::io::{Read};
use structopt::StructOpt;

// Labeled V0-VF -> 0 -> F, 0 -> 15
const VF: usize = 15;
type GeneralRegisters = [u8;16];
type AddressRegister = u16;
type Stack = Vec<u16>;

mod timer;
mod memory;

use timer::Timer;
use memory::Memory;

#[derive(Debug, StructOpt)]
#[structopt(name = "chip8", about = "Chip-8 Emulator")]
struct Opt {
    #[structopt(name = "ROM", parse(from_os_str))]
    rom: PathBuf
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let memory = Memory::from_file(opt.rom)?;

    println!("{:#?}", memory);

    // let mut t = Timer::new();
    // t.start();

    Ok(())
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_registers_initialize_to_zero() {
        let registers = GeneralRegisters::default();
        registers.iter().for_each(|&r| assert_eq!(r, 0));
    }

    #[test]
    fn test_vf_is_addressable() {
        let registers = GeneralRegisters::default();
        assert_eq!(registers[VF], 0);
    }
}