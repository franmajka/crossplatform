fn convert_to_be(num: u32) -> u32 {
  let bytes = num.to_le_bytes();
  u32::from_be_bytes(bytes)
}

fn convert_to_le(num: u32) -> u32 {
  let bytes = num.to_be_bytes();
  u32::from_le_bytes(bytes)
}

fn my_convert(num: u32) -> u32 {
  let mut res = 0;

  for i in 0..4 {
    res <<= 8;
    res |= (num >> (i * 8)) & 0xff;
  }

  res
}

fn main() {
  let numbers = vec![
    0xAA_BB_00_FF,
    0x00_FF_AA_BB,
    0x00_00_00_01,
    0xFF_FF_FF_00,
    0x11_22_33_44,
    0x55_66_77_88,
    0x99_AA_BB_CC,
    0xDD_EE_FF_00,
    0x12_34_56_78,
    0x9A_BC_DE_F0,
  ];

  for num in numbers {
    println!("0x{:08X} -> 0x{:08X}", num, convert_to_be(num));
    println!("0x{:08X} -> 0x{:08X}", num, my_convert(num));
    println!();
  }
}
