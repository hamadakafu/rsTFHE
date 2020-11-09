use crate::{tlwe, torus};
use std::num::Wrapping;

#[cfg(test)]
mod tests;

fn identity_key_switch(
    t: usize,
    basebit: usize,
    level1: tlwe::TLWE,
    ks: Vec<Vec<tlwe::TLWE>>,
) -> tlwe::TLWE {
    let n = ks[0][0].a.coef.len();
    let N = level1.a.coef.len();
    debug_assert_eq!(n, ks.len());
    debug_assert_eq!(t, ks[0].len());

    let mut level0 = tlwe::TLWE {
        a: torus::Torus01Vec::new_with_fix(vec![Wrapping(0); n]),
        b: level1.b,
    };

    let offset = 1 << (32 - (1 + basebit)); // basebitより1つ小さい桁で四捨五入するため
    for i in 0..N {
        let ai = level1.a.coef[i].fix + Wrapping(offset);
        for j in 0..t {
            let aik = (ai.0 >> (32 - (j + 1) * basebit)) & (2_u32.pow(basebit as u32) - 1);
            level0.a -= &ks[i][j].a * (aik as i64);
            level0.b -= ks[i][j].b * (aik as i64);
        }
    }
    return level0;
}
