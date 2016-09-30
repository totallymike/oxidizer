use system::System;
pub trait AddressingMode {
  fn load(&self, system: &mut System) -> u16;
}

pub struct AbsoluteAddressingMode {
  pub val: usize,
}

impl AddressingMode for AbsoluteAddressingMode {
  fn load(&self, system: &mut System) -> u16 {

    system.read_memory_address(self.val)
  }
}
