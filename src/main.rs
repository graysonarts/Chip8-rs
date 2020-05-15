#![feature(no_more_cas)]
#![allow(dead_code)]

#[macro_use] extern crate log;
use pretty_env_logger;

use std::path::PathBuf;
use structopt::StructOpt;

mod types;
mod memory;
mod timer;
mod opcodes;
mod errors;

use memory::Memory;
use timer::Timer;
use types::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "chip8", about = "Chip-8 Emulator")]
struct Opt {
    #[structopt(name = "ROM", parse(from_os_str))]
    rom: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let opt = Opt::from_args();

    // Load the rom
    let mut memory = Memory::from_file(opt.rom)?;
    let mut registers = GeneralRegisters::default();
    let mut i = AddressRegister::default();
    let mut stack : Stack = Vec::new();
    let mut pc = ProgramCounter::default();

    println!("{:#?}", memory);

    // Initialize Timers
    let mut t = Timer::new();
    t.start();

    loop {
        debug!("PC={:04x} I={:04x} REG={:?}", pc, i, registers);
        let hb = memory[pc];
        let lb = memory[pc+1];
        let oc = u16::from_be_bytes([hb, lb]);
        pc += 2;

        opcodes::OpCode::from(oc).execute(&mut pc, &mut memory, &mut registers, &mut i, &mut stack)?;
    }
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
