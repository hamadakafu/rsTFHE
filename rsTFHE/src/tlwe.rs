use rand::{self, Rng};
use std::num::Wrapping;

use crate::params;
use crate::torus::{Torus01, Torus01Vec};

#[cfg(test)]
mod tests;

pub fn encrypt(m: u32) -> (TLWE, Vec<i64>, Torus01) {
    assert!(m == 0 || m == 1);
    let a = gen_a(params::n);
    let s = gen_s(params::n);
    let e = gen_e();
    let b = &a * &s
        + if m == 0 {
            -Torus01::new_with_float(params::mu)
        } else {
            Torus01::new_with_float(params::mu)
        }
        + e;
    return (TLWE { a, b }, s, e);
}

pub fn decrypt(c: TLWE, s: Vec<i64>) -> u32 {
    let b_hat = &c.a * &s;
    let result = c.b - b_hat;
    // マイナス
    if (result.fix >> (32 - 1)) == Wrapping(1) {
        0
    } else {
        1
    }
}

#[derive(Debug, Clone)]
pub struct TLWE {
    pub a: Torus01Vec,
    pub b: Torus01,
}

fn gen_s(size: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(1, 2)).collect()
}

fn gen_a(size: usize) -> Torus01Vec {
    let mut rng = rand::thread_rng();
    Torus01Vec::new_with_fix(
        (0..size)
            .map(|_| Wrapping(rng.gen_range(0, u32::MAX)))
            .collect(),
    )
}

fn gen_e() -> Torus01 {
    Torus01::sample(params::alpha)
}
