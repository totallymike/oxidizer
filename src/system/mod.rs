use std::io::prelude::*;
use std::io::Cursor;
use std::io::SeekFrom;
use std::result;
use byteorder::{BigEndian,ReadBytesExt,WriteBytesExt};

use rom;
use m68k;
use m68k::{M68k,OpCode};

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
    let location = self.cpu.pc_register;
    println!("location: ${:X}", location);
    let instruction = self.read_next_word() as u16;
    let opcode = match OpCode::interpret(instruction) {
      Some(opcode) => opcode,
      None => { return Err("Fuck"); },
    };

    println!("instruction: {}", opcode.instruction);
    println!("opcode {:#018b}", instruction);
    match opcode.instruction {
      m68k::TST_L => {
        let addressing_mode = opcode.addressing_mode();

        let data = addressing_mode
          .load(self)
          .read_i32::<BigEndian>()
          .unwrap();
        self.tst_l(data);
        Ok(true)
      }
      m68k::TST_W => {
        let addressing_mode = opcode.addressing_mode();

        let mut data = addressing_mode
          .load(self)
          .read_i16::<BigEndian>()
          .unwrap();
        self.tst_w(data);

        Ok(true)
      }
      m68k::BNE => {
        println!("CC Register {:018b}", self.cpu.cc_register);
        if !self.cpu.zero_bit() {
          println!("not zero, branching!");
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

  pub fn read_next_word(&mut self) -> i16 {
    let address = self.cpu.pc_register as usize;
    self.cpu.pc_register += 2;
    self.read_memory_address(address)
  }

  pub fn read_next_long(&mut self) -> u32 {
    let address = self.cpu.pc_register as usize;
    self.cpu.pc_register += 4;
    self.read_ulong(address)
  }

  pub fn read_ulong(&mut self, address: usize) -> u32 {
    let mut memory = &self.memory[address..];

    memory.read_u32::<BigEndian>().unwrap()
  }

  pub fn read_memory_address(&self, address: usize) -> i16 {
    let mut memory = &self.memory[address..];

    memory.read_i16::<BigEndian>().unwrap()
  }

  fn tst_l(&mut self, operand: i32) {
    println!("VALUE: {:#06x}", operand);

    let extend_flag = self.cpu.cc_register & 0b10000;
    let new_flags = extend_flag | match operand {
      0i32 => 0b0100,
      n if n < 0 => 0b1000,
      _          => 0b0000,
    };
    self.cpu.set_cc_flags(new_flags);
    println!("NEW CC {:#018x}", self.cpu.cc_register);
  }

  fn tst_w(&mut self, operand: i16) {
    println!("VALUE: {:#06x}", operand);

    let extend_flag = self.cpu.cc_register & 0b10000;
    let new_flags = extend_flag | match operand {
      0i16 => 0b0100,
      n if n < 0 => 0b1000,
      _          => 0b0000,
    };
    self.cpu.set_cc_flags(new_flags);
    println!("{:#018x}", self.cpu.cc_register);
  }
}
