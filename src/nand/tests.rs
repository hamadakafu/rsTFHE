use quickcheck_macros::quickcheck;

use super::*;

#[quickcheck]
fn test_nand(b1: bool, b2: bool) -> bool {
    let expect = !(b1 & b2);
    let real = nand(b1, b2);
    if expect != real {
        dbg!(b1, b2, expect, real);
    } else {
        dbg!("ok", b1, b2, expect, real);
    }
    expect == real
}

#[ignore]
#[quickcheck]
fn test_nand_lvl1(b1: bool, b2: bool) -> bool {
    // 暗号化
    let n = params::n;
    let mu_bit = params::mu_bit;
    let s_tlwe = tlwe::gen_s(n);
    let (t1, _) = tlwe::encrypt_bin(b1 as u32, &s_tlwe, 1.0 / 2.0_f64.powi(mu_bit as i32));
    let (t2, _) = tlwe::encrypt_bin(b2 as u32, &s_tlwe, 1.0 / 2.0_f64.powi(mu_bit as i32));
    let adjustment = tlwe::TLWE {
        a: torus::Torus01Vec::new_with_fix(vec![Wrapping(0); n]),
        b: torus::Torus01::new_with_fix(Wrapping(1 << (32 - mu_bit))),
    };
    let c_tlwe = -t1 - t2 + adjustment;

    // blind rotate
    let N = params::N;
    let N_bit = params::N_bit;
    let l = params::l;
    let bg = params::bg;
    let bgbit = params::bgbit;
    let s_trlwe = trlwe::gen_s(N);
    let trgsws = blind_rotate::encrypt_tlwe_s(&s_tlwe, &s_trlwe, l, bg, bgbit);
    let trlwe = blind_rotate::blind_rotate(c_tlwe, trgsws, mu_bit as u32, N as u32, N_bit as u32);

    // sample extract index
    let tlwe_lv1 = trlwe.sample_extract_index(0);

    let real = match tlwe::decrypt(tlwe_lv1, &s_trlwe) {
        0 => false,
        1 => true,
        _ => panic!("jfiaofjafj"),
    };
    let expect = !(b1 && b2);
    if expect != real {
        dbg!(b1, b2, expect, real);
    } else {
        dbg!("ok", b1, b2, expect, real);
    }
    expect == real
}
