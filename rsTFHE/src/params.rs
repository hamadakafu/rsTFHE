// 固定浮動小数点の幅
pub const w: usize = 32;

pub const mu: f64 = 0.125;
pub const mu_bit: usize = 3;

// TLWE
/// 2 ** -15
pub const alpha: f64 = 3.0517578125e-05;
pub const n: usize = 10;

// TRLWE
pub const N: usize = 1024;
pub const N_bit: usize = 10;
/// 2 ** -25
pub const alpha_bk: f64 = 2.9802322387695312e-08;

// TRGSW
pub const l:usize = 3;
pub const bg:usize = 64;
pub const bgbit: usize = 6;
