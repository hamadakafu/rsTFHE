use rand_distr::{Distribution, Normal};
use std::{
    cmp::Ordering,
    f64::consts::PI,
    num::Wrapping,
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
};

#[cfg(feature = "fft")]
use rustfft::num_complex::Complex;
#[cfg(feature = "fft")]
use rustfft::num_traits::Zero;
#[cfg(feature = "fft")]
use rustfft::FFTplanner;

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

    /// 32bitの固定小数点として扱う
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

impl AddAssign<Torus01> for Torus01 {
    fn add_assign(&mut self, rhs: Torus01) {
        let tmp = *self + rhs;
        self.fix = tmp.fix;
        self.float = tmp.float
    }
}
impl Sub<Torus01> for Torus01 {
    type Output = Self;
    fn sub(self, rhs: Torus01) -> Self::Output {
        self + (-rhs)
    }
}

impl SubAssign<Torus01> for Torus01 {
    fn sub_assign(&mut self, rhs: Torus01) {
        let tmp = *self + (-rhs);
        self.fix = tmp.fix;
        self.float = tmp.float;
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
        if self.fix.0 == 0 {
            return self;
        }
        Torus01::new_with_fix(Wrapping(((1 << 32) - self.fix.0 as u64) as u32))
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
    pub coef: Vec<Torus01>,
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

impl AddAssign<Torus01Vec> for Torus01Vec {
    fn add_assign(&mut self, rhs: Torus01Vec) {
        for i in 0..self.coef.len() {
            self.coef[i] += rhs.coef[i];
        }
    }
}

impl SubAssign<Torus01Vec> for Torus01Vec {
    fn sub_assign(&mut self, rhs: Torus01Vec) {
        for i in 0..self.coef.len() {
            self.coef[i] -= rhs.coef[i];
        }
    }
}

impl Neg for Torus01Vec {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Torus01Vec {
            coef: self.coef.into_iter().map(|t| -t).collect(),
        }
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

/// スカラ倍
impl Mul<i64> for &Torus01Vec {
    type Output = Torus01Vec;

    fn mul(self, rhs: i64) -> Self::Output {
        Torus01Vec::new_with_torus(self.coef.iter().map(|t| *t * rhs).collect())
    }
}

#[derive(Debug, Clone)]
pub struct Torus01Poly {
    pub coef: Vec<Torus01>,
}

impl Torus01Poly {
    pub fn new_with_fix(coef: Vec<Wrapping<u32>>) -> Self {
        Torus01Poly {
            coef: coef
                .into_iter()
                .map(|fix| Torus01::new_with_fix(fix))
                .collect(),
        }
    }

    pub fn new_with_torus(coef: Vec<Torus01>) -> Self {
        Torus01Poly { coef }
    }
}

impl Eq for Torus01Poly {}

impl PartialEq for Torus01Poly {
    fn eq(&self, other: &Self) -> bool {
        self.coef == other.coef
    }
}

impl Add<&Torus01Poly> for &Torus01Poly {
    type Output = Torus01Poly;
    fn add(self, rhs: &Torus01Poly) -> Self::Output {
        assert_eq!(self.coef.len(), rhs.coef.len());
        Torus01Poly {
            coef: self
                .coef
                .iter()
                .zip(rhs.coef.iter())
                .map(|(l, r)| *l + *r)
                .collect(),
        }
    }
}

impl Sub<&Torus01Poly> for &Torus01Poly {
    type Output = Torus01Poly;
    fn sub(self, rhs: &Torus01Poly) -> Self::Output {
        assert_eq!(self.coef.len(), rhs.coef.len());

        Torus01Poly {
            coef: self
                .coef
                .iter()
                .zip(rhs.coef.iter())
                .map(|(l, r)| *l - *r)
                .collect(),
        }
    }
}

#[cfg(not(feature = "fft"))]
impl Mul<&Vec<i64>> for &Torus01Poly {
    type Output = Torus01Poly;
    fn mul(self, rhs: &Vec<i64>) -> Self::Output {
        assert_eq!(self.coef.len(), rhs.len());
        let mut coef = vec![Torus01::new_with_fix(Wrapping(0)); self.coef.len() * 2 - 1];
        // TODO: fft使う
        for (li, le) in self.coef.iter().enumerate() {
            for (ri, re) in rhs.iter().enumerate() {
                coef[li + ri] += *le * *re;
            }
        }
        for i in (0..self.coef.len() - 1).rev() {
            let tmp = coef.pop().unwrap();
            coef[i] -= tmp;
        }
        assert_eq!(coef.len(), self.coef.len());
        Torus01Poly::new_with_torus(coef)
    }
}

#[cfg(feature = "fft")]
impl Mul<&Vec<i64>> for &Torus01Poly {
    type Output = Torus01Poly;
    fn mul(self, rhs: &Vec<i64>) -> Self::Output {
        let len = self.coef.len();
        let w = Complex::new(0.0, -2.0 * PI / len as f64).exp();

        let mut lfft: Vec<Complex<f64>> = self
            .coef
            .iter()
            .enumerate()
            .map(|(i, l)| w.powf(i as f64 / 2.0) * Complex::new((l.fix.0 as f64), 0.0))
            .collect();
        let mut lfft_out: Vec<Complex<f64>> = vec![Complex::zero(); len];
        let mut rfft: Vec<Complex<f64>> = rhs
            .iter()
            .enumerate()
            .map(|(i, r)| w.powf(i as f64 / 2.0) * Complex::new(*r as f64, 0.0))
            .collect();
        let mut rfft_out: Vec<Complex<f64>> = vec![Complex::zero(); len];

        let mut planner = FFTplanner::new(false);
        let fft = planner.plan_fft(len);
        fft.process(&mut lfft, &mut lfft_out);
        fft.process(&mut rfft, &mut rfft_out);

        let mut planner = FFTplanner::new(true);

        let fft = planner.plan_fft(len);
        let mut result_fft: Vec<Complex<f64>> = lfft_out
            .into_iter()
            .zip(rfft_out)
            .enumerate()
            .map(|(i, (l, r))| l * r)
            .collect();
        let mut result_fft_out: Vec<Complex<f64>> = vec![Complex::zero(); len];
        fft.process(&mut result_fft, &mut result_fft_out);
        let coef = result_fft_out
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                let mut tmp = (c / w.powf(i as f64 / 2.0)).re.round() as i128;
                tmp /= len as i128;
                tmp = tmp % u32::MAX as i128;
                if tmp < 0 {
                    tmp = u32::MAX as i128 + tmp;
                }

                Torus01::new_with_fix(Wrapping(tmp as u32))
            })
            .collect();
        Torus01Poly::new_with_torus(coef)
    }
}
