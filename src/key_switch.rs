use crate::{tlwe, torus};
use std::num::Wrapping;

#[cfg(test)]
mod tests;

/// level1: key switchしたいtlwe
/// ks: s_trlweをs_tlweでビット毎に暗号化したやつ
/// t: a_iを何本のa_ijにDecompositionするか
pub fn identity_key_switch(
    level1: tlwe::TLWE,
    ks: Vec<Vec<tlwe::TLWE>>,
    t: usize,
    basebit: usize,
) -> tlwe::TLWE {
    let n = ks[0][0].a.coef.len();
    let N = level1.a.coef.len();
    debug_assert_eq!(N, ks.len());
    debug_assert_eq!(t, ks[0].len());

    let mut level0 = tlwe::TLWE {
        a: torus::Torus01Vec::new_with_fix(vec![Wrapping(0); n]),
        b: level1.b,
    };

    // basebitより1つ小さい桁で四捨五入するため
    let offset = Wrapping(1 << (32 - (1 + basebit * t)));

    for i in 0..N {
        let ai = level1.a.coef[i].fix + offset;
        for j in 0..t {
            let aik = (ai.0 >> (32 - (j + 1) * basebit)) & (2_u32.pow(basebit as u32) - 1);
            level0.a -= &ks[i][j].a * (aik as i64);
            level0.b -= ks[i][j].b * (aik as i64);
        }
    }
    return level0;
}

/// s_trlweをDecomposition的なことをしながら、bit毎にs_tlweを使って暗号化する
pub fn gen_ks(
    s_trlwe: &Vec<i64>,
    s_tlwe: &Vec<i64>,
    t: usize,
    basebit: usize,
) -> Vec<Vec<tlwe::TLWE>> {
    let n = s_tlwe.len();
    let N = s_trlwe.len();
    (0..N)
        .map(|i| {
            (1..t + 1)
                .map(|j| {
                    let m = match s_trlwe[i] {
                        0 => torus::Torus01::new_with_fix(Wrapping(0)),
                        1 => {
                            let fix = Wrapping(1 << (32 - (j * basebit)));
                            torus::Torus01::new_with_fix(fix)
                        }
                        _ => {
                            panic!("wtffffff");
                        }
                    };
                    tlwe::encrypt_torus(m, s_tlwe).0
                })
                .collect()
        })
        .collect()
}
