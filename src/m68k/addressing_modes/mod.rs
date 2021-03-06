use system::System;

pub trait AddressingMode {
  fn load(&self, system: &mut System) -> i16;
}

pub struct AbsoluteAddressingMode {
  pub val: usize,
  pub size: super::SizeField,
}

impl AddressingMode for AbsoluteAddressingMode {
  fn load(&self, system: &mut System) -> i16 {

    system.read_memory_address(self.val)
  }
}
