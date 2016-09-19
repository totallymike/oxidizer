bitflags! {
  pub flags SizeField: u16 {
    const BYTE = 0b00 << 6,
    const WORD = 0b01 << 6,
    const LONG = 0b10 << 6,
  }
}
bitflags! {
  pub flags OpCodes: u16 {
    const TST = 0b01001010 << 8,
    const TST_L = TST.bits | LONG.bits,
  }
}
