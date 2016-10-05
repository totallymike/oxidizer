use std::fmt;

use system::System;

pub mod addressing_modes;

bitflags! {
  pub flags SizeField: u16 {
    const BYTE = 0b00 << 6,
    const WORD = 0b01 << 6,
    const LONG = 0b10 << 6,
  }
}

bitflags! {
  pub flags Instructions: u16 {
    const TST = 0b01001010 << 8,
    const TST_W = TST.bits | WORD.bits,
    const TST_L = TST.bits | LONG.bits,
    const BNE = 0b01100110 << 8,
  }
}

bitflags! {
  pub flags AddressingModeFlags: u16 {
    const ABSOLUTE_ADDRESSING_LONG_MODE = 0b111001,
    const ABSOLUTE_ADDRESSING_WORD_MODE = 0b111000,
  }
}

pub trait AddressingMode {
  fn load<'a>(&self, system: &'a mut System) -> &'a [u8];
}

pub trait AbsoluteAddressingMode : AddressingMode {
  fn address(&self, system: &mut System) -> usize;
}

pub struct ErrorMode;

impl AddressingMode for ErrorMode {
  fn load<'a>(&self, _system: &'a mut System) -> &'a [u8] {
    panic!("Not supposed to happen!");
  }
}

pub struct AbsoluteLongAddressingMode;

impl AddressingMode for AbsoluteLongAddressingMode {
  fn load<'a>(&self, system: &'a mut System) -> &'a [u8] {
    let address = self.address(system);
    println!("Reading address ${:x}", address);
    &system.memory[address..]
  }
}

impl AbsoluteAddressingMode for AbsoluteLongAddressingMode {
  fn address(&self, system: &mut System) -> usize {
    system.read_next_long() as usize
  }
}

pub struct OpCode {
  pub instruction: Instructions,
  raw_value: u16,
}

impl OpCode {
  pub fn interpret(bits: u16) -> Option<OpCode> {
    let instruction = Instructions::from_bits_truncate(bits);

    Some(OpCode {
      instruction: instruction,
      raw_value: bits
    })
  }

  pub fn addressing_mode(&self) -> Box<AddressingMode> {
    match AddressingModeFlags::from_bits_truncate(self.raw_value) {
      ABSOLUTE_ADDRESSING_LONG_MODE => Box::new(AbsoluteLongAddressingMode),
      _ => Box::new(ErrorMode)
    }
  }
}

impl fmt::Display for Instructions {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      TST_L => write!(f, "tst.l"),
      TST_W => write!(f, "tst.w"),
      BNE => write!(f, "bne"),
      _ => write!(f, "instruction not found"),
    }
  }
}

bitflags! {
  flags ConditionCodes: u16 {
    const CARRY    = 0b00001,
    const OVERFLOW = 0b00010,
    const ZERO     = 0b00100,
    const NEGATIVE = 0b01000,
    const EXTEND   = 0b10000,
  }
}

pub struct M68k {
  pub cc_register: u16,
  pub pc_register: u16,
  data_registers: [u16; 8],
  address_registers: [u16; 8],
}

impl M68k {
  pub fn new() -> M68k {
    M68k {
      cc_register: 0,
      pc_register: 0,
      data_registers: [0; 8],
      address_registers: [0; 8],
    }
  }

  pub fn set_cc_flags(&mut self, flags: u16) {
    self.cc_register = flags;
  }

  pub fn set_zero_flag(&mut self) {
    self.cc_register = ZERO.bits;
  }

  pub fn zero_bit(&self) -> bool {
    ConditionCodes::from_bits(self.cc_register)
      .unwrap()
      .contains(ZERO)
  }

  pub fn extend_bit(&self) -> bool {
    ConditionCodes::from_bits(self.cc_register)
      .unwrap()
      .contains(EXTEND)
  }
}

#[cfg(test)]
mod tests {
  use super::M68k;
  #[test]
  fn set_cc_flags_works() {
    let mut cpu = M68k::new();
    assert_eq!(cpu.cc_register, 0);

    cpu.set_cc_flags(0b100);
    assert_eq!(cpu.cc_register, 0b100);
    cpu.set_cc_flags(0b1010);
    assert_eq!(cpu.cc_register, 0b1010);
  }

  #[test]
  fn whats_up_with_bitflags() {
    let foo: u16 = 0b111001;
    match super::AddressingModeFlags::from_bits_truncate(foo) {
      super::ABSOLUTE_ADDRESSING_LONG_MODE => assert!(true),
      super::ABSOLUTE_ADDRESSING_WORD_MODE => assert!(false),
      super::AddressingModeFlags{ .. } => assert!(false),
    }
  }
}
