use quickcheck_macros::quickcheck;

use super::*;
use crate::{params, tlwe, torus};

#[test]
fn test_tmp() {
    let l = params::l;
    let bg = params::bg as u32;
    let bgbit = params::bgbit;
    let n = params::n;
    let N = params::N;
    let N_bit = params::N_bit;
    let mu = params::mu;
    let mu_bit = params::mu_bit;

    let tlwe_s = tlwe::gen_s(n);
    let trlwe_s = trlwe::gen_s(N);
    let trgsws = encrypt_tlwe_s(&tlwe_s, &trlwe_s, l, bg as usize, bgbit);
    let m_fix = Wrapping((1 << 30) - 10000);
    let (tlwe_c, _) = tlwe::encrypt_torus(torus::Torus01::new_with_fix(m_fix), &tlwe_s);

    let blind_trlwe = blind_rotate(tlwe_c, trgsws, mu_bit as u32, N as u32, N_bit as u32);
    // // 自明な暗号文
    // let blind_trlwe = blind_rotate(
    //     tlwe::TLWE {
    //         a: torus::Torus01Vec::new_with_fix(vec![Wrapping(0); n]),
    //         b: torus::Torus01::new_with_fix(m_fix),
    //     },
    //     trgsws,
    //     mu_bit as u32,
    //     N as u32,
    //     N_bit as u32,
    // );
    let blind_msg = trlwe::decrypt_bin(blind_trlwe, &trlwe_s);

    let rotate_count: u32 =
        (m_fix + Wrapping(1 << (31 - N_bit as u32 - 1))).0 >> (32 - N_bit as u32 - 1);
    if rotate_count >= N as u32 {
        let zeros = blind_msg
            .iter()
            .take_while(|b| **b == 0)
            .collect::<Vec<&u8>>();
        let ones = blind_msg
            .iter()
            .rev()
            .take_while(|b| **b == 1)
            .collect::<Vec<&u8>>();

        dbg!(rotate_count, m_fix, zeros.len(), ones.len());
        debug_assert!(zeros.len() > 0 && ones.len() > 0);
    } else if rotate_count < N as u32 {
        let zeros = blind_msg
            .iter()
            .rev()
            .take_while(|b| **b == 0)
            .collect::<Vec<&u8>>();
        let ones = blind_msg
            .iter()
            .take_while(|b| **b == 1)
            .collect::<Vec<&u8>>();

        dbg!(rotate_count, m_fix, zeros.len(), ones.len());
        debug_assert!(zeros.len() > 0 && ones.len() > 0);
    } else {
        panic!("jafiowefjpwafeopjafoj")
    }
}

#[quickcheck]
fn test_blind_rotate_bin(m: u32) -> bool {
    // 時間かかるのでちょっとだけテスト
    if m > 10 {
        return true;
    }
    let m = m % 2;

    let l = params::l;
    let bg = params::bg as u32;
    let bgbit = params::bgbit;
    let n = params::n;
    let N = params::N;
    let N_bit = params::N_bit;
    let mu = params::mu;
    let mu_bit = params::mu_bit;

    let tlwe_s = tlwe::gen_s(n);
    let (tlwe_c, _) = tlwe::encrypt_bin(m, &tlwe_s, mu);

    let m_hat = tlwe::decrypt(tlwe_c.clone(), &tlwe_s);
    dbg!(m, m_hat);
    assert_eq!(m, m_hat);

    let trlwe_s = trlwe::gen_s(N);
    let trgsws = encrypt_tlwe_s(&tlwe_s, &trlwe_s, l, bg as usize, bgbit);

    let blind_trlwe = blind_rotate(tlwe_c, trgsws, mu_bit as u32, N as u32, N_bit as u32);
    let blind_msg = trlwe::decrypt_bin(blind_trlwe, &trlwe_s);

    if m == 1 {
        let zeros = blind_msg
            .iter()
            .take_while(|b| **b == 0)
            .collect::<Vec<&u8>>();
        let ones = blind_msg
            .iter()
            .rev()
            .take_while(|b| **b == 1)
            .collect::<Vec<&u8>>();
        dbg!(zeros.len(), ones.len());
        return 250 <= ones.len() && ones.len() <= 260;
    } else if m == 0 {
        let zeros = blind_msg
            .iter()
            .take_while(|b| **b == 0)
            .collect::<Vec<&u8>>();
        let ones = blind_msg
            .iter()
            .rev()
            .take_while(|b| **b == 1)
            .collect::<Vec<&u8>>();

        if 760 <= zeros.len() && zeros.len() <= 770 {
            return true;
        } else {
            let zeros = blind_msg
                .iter()
                .rev()
                .take_while(|b| **b == 0)
                .collect::<Vec<&u8>>();
            let ones = blind_msg
                .iter()
                .take_while(|b| **b == 1)
                .collect::<Vec<&u8>>();
            dbg!(zeros.len(), ones.len());
            return false;
        }
    } else {
        panic!("jafiowefjpwafeopjafoj")
    }
}

#[quickcheck]
fn test_blind_rotate_torus(m: u32) -> bool {
    // 時間かかるのでちょっとだけテスト
    let mm = vec![
        (1 << (32 - 3)) + 100,
        (1 << (32 - 3)) + (1 << (32 - 2)),
        (Wrapping(0 as u32) - Wrapping(1 << (32 - 3))).0 - 100,
        (Wrapping(0 as u32) - Wrapping((1 << (32 - 3)) + (1 << (32 - 2)))).0,
    ];
    if m as usize >= mm.len() {
        return true;
    }
    let idx = m;
    dbg!(idx);
    let m = mm[m as usize];

    let l = params::l;
    let bg = params::bg as u32;
    let bgbit = params::bgbit;
    let n = params::n;
    let N = params::N;
    let N_bit = params::N_bit;
    let mu = params::mu;
    let mu_bit = params::mu_bit;

    let tlwe_s = tlwe::gen_s(n);
    let (tlwe_c, _) = tlwe::encrypt_torus(torus::Torus01::new_with_fix(Wrapping(m)), &tlwe_s);
    let trlwe_s = trlwe::gen_s(N);
    let trgsws = encrypt_tlwe_s(&tlwe_s, &trlwe_s, l, bg as usize, bgbit);

    let blind_trlwe = blind_rotate(tlwe_c, trgsws, mu_bit as u32, N as u32, N_bit as u32);
    let blind_msg = trlwe::decrypt_bin(blind_trlwe, &trlwe_s);

    let rotate_count: u32 =
        (Wrapping(m) + Wrapping(1 << (31 - N_bit as u32 - 1))).0 >> (32 - N_bit as u32 - 1);

    if rotate_count >= N as u32 {
        let zeros = blind_msg
            .iter()
            .take_while(|b| **b == 0)
            .collect::<Vec<&u8>>();
        let ones = blind_msg
            .iter()
            .rev()
            .take_while(|b| **b == 1)
            .collect::<Vec<&u8>>();

        dbg!(rotate_count, m, zeros.len(), ones.len());
        return zeros.len() > 0 && ones.len() > 0;
    } else if rotate_count < N as u32 {
        let zeros = blind_msg
            .iter()
            .rev()
            .take_while(|b| **b == 0)
            .collect::<Vec<&u8>>();
        let ones = blind_msg
            .iter()
            .take_while(|b| **b == 1)
            .collect::<Vec<&u8>>();

        dbg!(rotate_count, m, zeros.len(), ones.len());
        return zeros.len() > 0 && ones.len() > 0;
    } else {
        panic!("jafiowefjpwafeopjafoj")
    }
}
