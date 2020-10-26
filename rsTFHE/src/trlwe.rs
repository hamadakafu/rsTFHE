use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::num::Wrapping;
use std::ops::{Add, Sub};

use crate::params;
use crate::tlwe;
use crate::torus::{Torus01, Torus01Poly, Torus01Vec};

#[cfg(test)]
mod tests;

/// 0か1を-1/8と1/8にマッピングして暗号化する
pub fn encrypt_bin(m: Vec<u8>, s: &Vec<i64>) -> (TRLWE, Torus01Poly) {
    for i in m.iter() {
        assert!(*i == 0 || *i == 1);
    }
    let N = s.len();
    let a = gen_a(N);
    let e = gen_e(N, params::alpha_bk);
    let b = &(&(&a * &s)
        + &Torus01Poly::new_with_torus(
            m.iter()
                .map(|e| Torus01::new_with_float(if *e == 0 { -params::mu } else { params::mu }))
                .collect(),
        ))
        + &e;
    (TRLWE { a, b }, e)
}

/// トーラス上の任意の固定浮動小数点を暗号化する
/// 復号ができない
pub fn encrypt_torus(m: Vec<u32>, s: &Vec<i64>) -> (TRLWE, Torus01Poly) {
    let N = s.len();
    let a = gen_a(N);
    let e = gen_e(N, params::alpha_bk);
    let b = &(&(&a * &s)
        + &Torus01Poly::new_with_torus(
            m.iter()
                .map(|e| Torus01::new_with_fix(Wrapping(*e)))
                .collect(),
        ))
        + &e;
    (TRLWE { a, b }, e)
}

pub fn decrypt_bin(c: TRLWE, s: &Vec<i64>) -> Vec<u8> {
    let b_hat = &c.a * s;
    let mut ans = Vec::with_capacity(s.len());
    for (left, right) in c.b.coef.iter().zip(b_hat.coef.iter()) {
        let tmp = *left - *right;
        ans.push(if (tmp.fix >> (32 - 1)) == Wrapping(1) {
            0
        } else {
            1
        });
    }
    ans
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct TRLWE {
    pub a: Torus01Poly,
    pub b: Torus01Poly,
}

impl TRLWE {
    /// idx番目の係数のTLWEを生成する
    pub fn sample_extract_index(self, idx: usize) -> tlwe::TLWE {
        let size = self.a.coef.len();
        assert!(size > idx);
        let mut a = Vec::with_capacity(size);
        for i in 0..size {
            if i <= idx {
                a.push(self.a.coef[idx - i])
            } else {
                a.push(-self.a.coef[size + idx - i])
            }
        }
        tlwe::TLWE {
            a: Torus01Vec::new_with_torus(a),
            b: self.b.coef[idx],
        }
    }
}

impl Add<&TRLWE> for &TRLWE {
    type Output = TRLWE;
    fn add(self, rhs: &TRLWE) -> Self::Output {
        TRLWE {
            a: &self.a + &rhs.a,
            b: &self.b + &rhs.b,
        }
    }
}

impl Sub<&TRLWE> for &TRLWE {
    type Output = TRLWE;
    fn sub(self, rhs: &TRLWE) -> Self::Output {
        TRLWE {
            a: &self.a - &rhs.a,
            b: &self.b - &rhs.b,
        }
    }
}

pub fn gen_a(size: usize) -> Torus01Poly {
    Torus01Poly::new_with_fix((0..size).map(|_| rand::random()).collect())
}

pub fn gen_s(size: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0, 2)).collect()
}

pub fn gen_e(size: usize, std_dev: f64) -> Torus01Poly {
    let normal = Normal::new(0., std_dev).unwrap();
    Torus01Poly::new_with_torus(
        (0..size)
            .map(|_| Torus01::new_with_float(normal.sample(&mut rand::thread_rng())))
            .collect(),
    )
}
