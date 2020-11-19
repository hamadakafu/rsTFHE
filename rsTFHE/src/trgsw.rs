use std::num::Wrapping;
use std::ops::Mul;

use crate::params;
use crate::torus;
use crate::trlwe;

#[cfg(test)]
mod tests;

/// 論文でのHに0の暗号文でマスクした行列
#[derive(Debug, Clone)]
pub struct TRGSW {
    pub H: Vec<trlwe::TRLWE>,
    l: usize,
    bg: usize,
    bgbit: usize,
}

impl TRGSW {
    /// l: l polynomials are composed of
    /// bg: 2の階乗が望ましい
    /// zeros: 0の暗号文 2l本
    /// TODO: binだけでなく多項式の平文でも成立する
    pub fn new_with_bin(
        l: usize,
        bg: u32,
        bgbit: usize,
        bin: u8,
        zeros: Vec<trlwe::TRLWE>,
    ) -> Self {
        assert!(bin == 0 || bin == 1);
        assert_eq!(2 * l, zeros.len());
        if bin == 0 {
            dbg!("zero");
            return TRGSW {
                H: zeros,
                l,
                bg: bg as usize,
                bgbit,
            };
        }

        let mut H = Vec::with_capacity(2 * l);

        let bg_polys: Vec<torus::Torus01Poly> = (1..l + 1)
            .map(|i| {
                let mut tmp: Vec<torus::Torus01> = (0..zeros[0].a.coef.len())
                    .map(|_| torus::Torus01::new_with_fix(Wrapping(0)))
                    .collect();
                tmp[0] = torus::Torus01::new_with_fix(Wrapping((bin as u32) << (32 - bgbit * i)));

                torus::Torus01Poly::new_with_torus(tmp)
            })
            .collect();
        for i in 0..l {
            let a = &zeros[i].a + &bg_polys[i];
            H.push(trlwe::TRLWE {
                a,
                b: zeros[i].b.clone(),
            });
        }
        for i in 0..l {
            let b = &zeros[i + l].b + &bg_polys[i];
            H.push(trlwe::TRLWE {
                a: zeros[i + l].a.clone(),
                b,
            });
        }
        // dbg!(&H);
        TRGSW {
            H,
            l,
            bg: bg as usize,
            bgbit,
        }
    }

    /// cmuxはtrgswが0か1かでzeroの
    pub fn cmux(self, zero: trlwe::TRLWE, one: trlwe::TRLWE) -> trlwe::TRLWE {
        &(self * &(&one - &zero)) + &zero
    }
}

impl Mul<&trlwe::TRLWE> for TRGSW {
    type Output = trlwe::TRLWE;
    /// external product
    fn mul(self, rhs: &trlwe::TRLWE) -> Self::Output {
        // FIXME: をDIしたい
        let a_dec = decomposition(self.l, self.bg as u32, self.bgbit, &rhs.a);
        let b_dec = decomposition(self.l, self.bg as u32, self.bgbit, &rhs.b);
        // TODO: fft
        let mut ab_dec = Vec::with_capacity(self.l * 2);
        ab_dec.extend(a_dec);
        ab_dec.extend(b_dec);
        assert_eq!(ab_dec.len(), self.H.len());
        let mut a_acc = torus::Torus01Poly::new_with_fix(vec![Wrapping(0); rhs.a.coef.len()]);
        let mut b_acc = torus::Torus01Poly::new_with_fix(vec![Wrapping(0); rhs.a.coef.len()]);
        for (idx, (ab, hh)) in ab_dec.into_iter().zip(self.H.iter()).enumerate() {
            // TODO: 並列にできる
            a_acc = &a_acc + &(&hh.a * &ab);
            b_acc = &b_acc + &(&hh.b * &ab);
        }
        trlwe::TRLWE { a: a_acc, b: b_acc }
    }
}

/// 暗号文tをdecompositionする
/// l * Nの行列
fn decomposition(l: usize, bg: u32, bgbit: usize, p: &torus::Torus01Poly) -> Vec<Vec<i64>> {
    let size = p.coef.len();

    // FIXME: ここoffset足しすぎだと思ったがどうなんだ?
    // let offset = Wrapping(32 - l as u32 * bgbit as u32 - 1);
    let mut offset = Wrapping(0);
    for i in 1..l + 1 {
        offset += Wrapping(bg / 2 * (1 << (32 - i * bgbit)));
    }

    let coef: Vec<Wrapping<u32>> = p.coef.iter().map(|a_i| a_i.fix + offset).collect();
    (1..l + 1)
        .map(|i| {
            (0..size)
                .map(|j| (coef[j].0 >> (32 - bgbit * i) & (bg - 1)) as i64 - (bg / 2) as i64)
                .collect()
        })
        .collect()
}
