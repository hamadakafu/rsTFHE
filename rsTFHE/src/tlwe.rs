use rand::{self, Rng};
use std::num::Wrapping;
use std::ops::{Add, Neg, Sub};

use crate::params;
use crate::torus::{Torus01, Torus01Vec};

#[cfg(test)]
mod tests;

/// mは0か1
/// n: 鍵長
/// mu: は1/8を渡す
/// return (暗号文, エラー)
pub fn encrypt_bin(m: u32, s: &Vec<i64>, mu: f64) -> (TLWE, Torus01) {
    debug_assert!(m == 0 || m == 1);
    debug_assert!(s.iter().all(|b| *b == 0 || *b == 1));
    let m_t = if m == 0 {
        -Torus01::new_with_float(mu)
    } else {
        Torus01::new_with_float(mu)
    };
    encrypt_torus(m_t, s)
}

/// torus上で暗号化する
pub fn encrypt_torus(t: Torus01, s: &Vec<i64>) -> (TLWE, Torus01) {
    let n = s.len();
    let a = gen_a(n);
    let e = gen_e();
    let b = &a * s + t + e;
    return (TLWE { a, b }, e);
}

pub fn decrypt(c: TLWE, s: &Vec<i64>) -> u32 {
    let b_hat = &c.a * s;
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

impl Add<TLWE> for TLWE {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.a += rhs.a;
        self.b += rhs.b;
        return self;
    }
}

impl Sub<TLWE> for TLWE {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.a -= rhs.a;
        self.b -= rhs.b;
        return self;
    }
}

/// aとbを全部符号反転させる
/// 平文を符号反転させることに相当する
impl Neg for TLWE {
    type Output = Self;
    fn neg(self) -> Self::Output {
        TLWE {
            a: -self.a,
            b: -self.b,
        }
    }
}

pub fn gen_s(size: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0, 2)).collect()
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
