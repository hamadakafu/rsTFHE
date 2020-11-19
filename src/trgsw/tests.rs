use quickcheck_macros::quickcheck;
use std::num::Wrapping;

use super::*;
use crate::params;
use crate::torus::Torus01Poly;

/// decompositionを完璧に戻すことができるのか?
/// -> できない bgで小さい値は丸められている
#[quickcheck]
fn test_decomposition(fixes: Vec<Wrapping<u32>>) -> bool {
    if fixes.len() == 0 {
        return true;
    }

    let N = params::N;
    let l = 3;
    let bg = 64;
    let bgbit = 6;

    let fixes: Vec<Wrapping<u32>> = fixes.into_iter().cycle().take(N).collect();
    let poly = Torus01Poly::new_with_fix(fixes);
    let decomp = decomposition(l, bg, bgbit, &poly);

    let h: Vec<Torus01Poly> = (1..l + 1)
        .map(|i| {
            let mut coef = vec![Wrapping(0); N];
            coef[0] = Wrapping(1 << (32 - bgbit * i));
            Torus01Poly::new_with_fix(coef)
        })
        .collect();
    let mut acc = Torus01Poly::new_with_fix(vec![Wrapping(0); N]);
    for (dd, hh) in decomp.iter().zip(h.iter()) {
        acc = &acc + &(hh * dd);
    }
    // if poly != acc {
    //     dbg!(&poly, &acc);
    // }
    for (real, expect) in acc.coef.iter().zip(poly.coef.iter()) {
        if real > expect {
            assert!((*real - *expect).fix.0 < (1 << (32 - bgbit * l)));
        } else {
            assert!((*expect - *real).fix.0 < (1 << (32 - bgbit * l)));
        }
    }
    return true;
}

#[quickcheck]
fn test_cmux(mut m: u8) -> bool {
    m %= 2;
    let l = params::l;
    let bg = params::bg;
    let bgbit = params::bgbit;
    let N = 4;

    use crate::trlwe;
    let s = trlwe::gen_s(N);
    let mut errors = torus::Torus01Poly::new_with_fix(vec![Wrapping(0); N]);
    let zeros = (0..l * 2)
        .map(|_| {
            let m = vec![0; N];
            let (c, e) = trlwe::encrypt_torus(m, &s);
            errors = &errors + &e;
            c
        })
        .collect();
    let trgsw = TRGSW::new_with_bin(l, bg as u32, bgbit, m, zeros);

    // 定数項が0のTRLWE
    let d_0 = {
        let m = vec![0; N];
        let (c, e) = trlwe::encrypt_bin(m, &s);
        c
    };
    assert_eq!(trlwe::decrypt_bin(d_0.clone(), &s)[0], 0);

    // 定数項が1のTRLWE
    let d_1 = {
        let mut m = vec![0; N];
        m[0] = 1;
        let (c, e) = trlwe::encrypt_bin(m, &s);
        c
    };
    assert_eq!(trlwe::decrypt_bin(d_1.clone(), &s)[0], 1);

    let c_trlwe = trgsw.cmux(d_0, d_1);
    let m_hats = trlwe::decrypt_bin(c_trlwe, &s);
    m == m_hats[0]
}
