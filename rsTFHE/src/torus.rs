use rand_distr::{Distribution, Normal};
use std::{
    cmp::Ordering,
    num::Wrapping,
    ops::{Add, AddAssign, Mul, Neg, Sub},
};

use crate::params;

#[cfg(test)]
mod tests;

/// floatはあとで消していい、計算の効率が悪いので
#[derive(Debug, Default, Copy, Clone)]
pub struct Torus01 {
    pub fix: Wrapping<u32>,
    pub float: f64,
}

impl Torus01 {
    /// 正負は問わない
    #[deprecated = "floatからは精度とかがめんどくさいので余り使うべきでない"]
    pub fn new_with_float(float: f64) -> Self {
        let fisize: i64 = float as i64;
        let mut below_decimal = float - (fisize as f64);
        if below_decimal < 0.0 {
            below_decimal = below_decimal + 1.0;
        }

        let fix = (below_decimal * (2_u64.pow(params::w as u32) as f64)) as u32;
        // dbg!(below_decimal, fix);

        Torus01 {
            fix: Wrapping(fix),
            float: below_decimal,
        }
    }

    /// 32bitの固定浮動小数点として扱う
    pub fn new_with_fix(fix: Wrapping<u32>) -> Self {
        Torus01 {
            fix: fix,
            float: fix.0 as f64 / 2_u64.pow(params::w as u32) as f64,
        }
    }

    /// moduler gauss
    pub fn sample(std_dev: f64) -> Self {
        let normal = Normal::new(0., std_dev).unwrap();
        Torus01::new_with_float(normal.sample(&mut rand::thread_rng()))
    }
}

impl Add<Torus01> for Torus01 {
    type Output = Self;

    fn add(self, rhs: Torus01) -> Self::Output {
        Torus01::new_with_fix(self.fix + rhs.fix)
    }
}

impl Sub<Torus01> for Torus01 {
    type Output = Self;
    fn sub(self, rhs: Torus01) -> Self::Output {
        self + (-rhs)
    }
}

impl AddAssign<Torus01> for Torus01 {
    fn add_assign(&mut self, rhs: Torus01) {
        let tmp = Torus01::new_with_fix(self.fix + rhs.fix);
        self.fix = tmp.fix;
        self.float = tmp.float
    }
}

impl Mul<i64> for Torus01 {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Torus01::new_with_fix(Wrapping(rhs as u32) * self.fix)
    }
}

impl Neg for Torus01 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Torus01::new_with_fix(Wrapping(u32::MAX) - self.fix)
    }
}

impl Eq for Torus01 {}
impl PartialEq for Torus01 {
    fn eq(&self, other: &Self) -> bool {
        self.fix == other.fix
    }
}

impl PartialOrd for Torus01 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.fix.partial_cmp(&other.fix)
    }
}

impl Ord for Torus01 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fix.cmp(&other.fix)
    }
}

#[derive(Debug, Clone)]
pub struct Torus01Vec {
    coef: Vec<Torus01>,
}

impl Torus01Vec {
    pub fn new_with_fix(coef: Vec<Wrapping<u32>>) -> Self {
        Torus01Vec {
            coef: coef.into_iter().map(|c| Torus01::new_with_fix(c)).collect(),
        }
    }

    pub fn new_with_torus(coef: Vec<Torus01>) -> Self {
        Torus01Vec { coef }
    }

    /// 正規分布を使う
    pub fn sample(size: usize, std_dev: f64) -> Self {
        let mut coef = Vec::with_capacity(size);
        let normal = Normal::new(0., std_dev).unwrap();
        for _ in 0..size {
            coef.push(Torus01::new_with_float(
                normal.sample(&mut rand::thread_rng()),
            ))
        }
        Torus01Vec { coef }
    }
}

/// 内積
impl Mul<&Vec<i64>> for &Torus01Vec {
    type Output = Torus01;

    fn mul(self, rhs: &Vec<i64>) -> Self::Output {
        assert_eq!(self.coef.len(), rhs.len());
        let mut acc = Torus01::new_with_fix(Wrapping(0));
        for (l, r) in self.coef.iter().zip(rhs.iter()) {
            acc += *l * *r;
        }
        return acc;
    }
}
