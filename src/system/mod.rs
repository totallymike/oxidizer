use std::io::prelude::*;
use std::io::Cursor;
use std::io::SeekFrom;
use std::result;
use byteorder::{BigEndian,ReadBytesExt,WriteBytesExt};

use rom;
use m68k;
use m68k::OpCodes;
use m68k::M68k;

pub struct System {
  pub cpu: M68k,
  pub memory: Box<[u8]>,
}

impl System {
  pub fn new() -> System {
    System {
      cpu: M68k::new(),
      memory: vec![0; 0x1FFFFFE].into_boxed_slice(),
    }
  }

  pub fn process_next(system: System) -> Result<System, &'static str> {
    let mut system = system;
    match system.process_next_instruction() {
      Ok(true) => Ok(system),
      Err(err) => Err(err),
      _ => Err("something happened")
    }
  }

  pub fn process_next_instruction(&mut self) -> result::Result<bool, &'static str> {
    let instruction = self.read_next_word();
    let opcode = OpCodes::from_bits_truncate(instruction);

    println!("instruction: {}", opcode);
    println!("opcode {:#018b}", instruction);
    match opcode {
      m68k::TST_L => {
        let address_high: u32 = self.read_next_word() as u32;
        let address_low = self.read_next_word();
        let address_high = address_high << 16;
        let address: usize = address_high as usize + address_low as usize;
        let operand: u16 = self.read_memory_address(address);
        self.set_cpu_flags(operand);
        println!("val in operand {:010x}", operand);
        Ok(true)
      }
      m68k::TST_W => {
        let address = self.read_next_word() as usize;
        let operand: u16 = self.read_memory_address(address);
        println!("val in operand {:010x}", operand);
        self.set_cpu_flags(operand);
        Ok(true)
      }
      m68k::BNE => {
        if !self.cpu.zero() {
          let address = instruction as u8;
          if address != 0xFF && address != 0x0 {
            let new_address = self.cpu.pc_register + address as u16;
            println!("{:#x}", new_address);
            self.cpu.pc_register += address as u16;
          }
        }
        Ok(true)
      }
      _ => { Err("instruction not implemented") }
    }
  }

  pub fn load_rom(system: System, rom: rom::Rom) -> System {
    let mut fd = rom.fd;
    fd.seek(SeekFrom::Start(0)).expect("Couldn't rewind!");
    let mut cursor = Cursor::new(system.memory);
    cursor.seek(SeekFrom::Start(0)).expect("hrm...");
    while let Ok(n) = fd.read_u16::<BigEndian>() {
      cursor.write_u16::<BigEndian>(n).expect("Write failed!");
    }

    System { memory: cursor.into_inner(), ..system }
  }

  fn read_next_word(&mut self) -> u16 {
    let address = self.cpu.pc_register as usize;
    self.cpu.pc_register += 2;
    self.read_memory_address(address)
  }

  fn read_memory_address(&self, address: usize) -> u16 {
    println!("READ FROM ${:06x}", address);
    use std::mem;
    let memory = &self.memory;
    let val: u16 = unsafe { mem::transmute([memory[address], memory[address + 1]]) };
    u16::from_be(val)
  }

  fn set_cpu_flags(&mut self, operand: u16) {
    match operand {
      0u16 => { self.cpu.set_zero_flag(); },
      _ => { }
    }
  }
}
