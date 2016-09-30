use std::fmt;

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
    const CARRY = 0b01,
    const OVERFLOW = 0b10,
    const ZERO = 0b100,
    const EXTEND = 0b1000,
  }
}

pub struct M68k {
  cc_register: u16,
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

  pub fn set_zero_flag(&mut self) {
    self.cc_register = ZERO.bits;
  }

  pub fn zero(&self) -> bool {
    ConditionCodes::from_bits(self.cc_register)
      .unwrap()
      .contains(ZERO)
  }
}
