use super::*;
use crate::{params, tlwe, torus, trgsw};
use quickcheck_macros::quickcheck;
#[quickcheck]
fn test_blind_rotate(m: u32) -> bool {
    // 時間かかるのでちょっとだけテスト
    if (m > 10) {
        return true;
    }
    let l = params::l;
    let bg = params::bg as u32;
    let bgbit = params::bgbit;
    let n = params::n;
    let N = params::N;
    let N_bit = params::N_bit;
    let mu = params::mu;
    let mu_bit = params::mu_bit;
    let m = m % 2;
    let (tlwe_c, tlwe_s, e) = tlwe::encrypt(m, n, mu);
    let trlwe_s = trlwe::gen_s(N);
    let trgsws: Vec<trgsw::TRGSW> = tlwe_s
        .iter()
        .map(|sbit| {
            debug_assert!(*sbit == 0 || *sbit == 1);

            let zeros = (0..l * 2)
                .map(|_| {
                    let m = vec![0; N];
                    let (c, _) = trlwe::encrypt_torus(m, &trlwe_s);
                    c
                })
                .collect();
            trgsw::TRGSW::new_with_bin(l, bg, bgbit, *sbit as u8, zeros)
        })
        .collect();
    let blind_trlwe = blind_rotate(tlwe_c, trgsws, mu_bit as u32, N as u32, N_bit as u32);
    let blind_msg = trlwe::decrypt_bin(blind_trlwe, &trlwe_s);
    let mut one_count = 0;
    let mut zero_count = 0;
    for bin in blind_msg {
        if bin == 0 {
            zero_count += 1;
        } else if bin == 1 {
            one_count += 1;
        } else {
            panic!("hgoeheog");
        }
    }
    dbg!(m, zero_count, one_count);
    if m == 0 {
        return 250 <= zero_count && zero_count <= 260;
    } else {
        return 250 <= one_count && one_count <= 260;
    }
}
