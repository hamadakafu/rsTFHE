use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::num::Wrapping;

use crate::params;
use crate::tlwe;
use crate::torus::{Torus01, Torus01Poly, Torus01Vec};

#[cfg(test)]
mod tests;

pub fn encrypt(m: Vec<u8>) -> (TRLWE, Vec<i64>, Torus01Poly) {
    for i in m.iter() {
        assert!(*i == 0 || *i == 1);
    }
    let a = gen_a(params::N);
    let s = gen_s(params::N);
    let e = gen_e(params::N, params::alpha_bk);
    let b = &a * &s
        + &Torus01Poly::new_with_torus(
            m.iter()
                .map(|e| Torus01::new_with_float(if *e == 0 { -params::mu } else { params::mu }))
                .collect(),
        )
        + &e;
    (TRLWE { a, b }, s, e)
}

pub fn decrypt(c: TRLWE, s: Vec<i64>) -> Vec<u8> {
    let b_hat = &c.a * &s;
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
pub struct TRLWE {
    a: Torus01Poly,
    b: Torus01Poly,
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

fn gen_a(size: usize) -> Torus01Poly {
    Torus01Poly::new_with_fix((0..size).map(|_| rand::random()).collect())
}

fn gen_s(size: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0, 2)).collect()
}

fn gen_e(size: usize, std_dev: f64) -> Torus01Poly {
    let normal = Normal::new(0., std_dev).unwrap();
    Torus01Poly::new_with_torus(
        (0..size)
            .map(|_| Torus01::new_with_float(normal.sample(&mut rand::thread_rng())))
            .collect(),
    )
}
