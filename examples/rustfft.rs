#[cfg(feature = "fft")]
use rustfft::num_complex::Complex;
#[cfg(feature = "fft")]
use rustfft::num_traits::Zero;
#[cfg(feature = "fft")]
use rustfft::FFTplanner;
use std::sync::Arc;

#[cfg(not(feature = "fft"))]
fn main() {}

#[cfg(feature = "fft")]
fn main() {
    let left = vec![3, 2, 1];
    let right = vec![5, 0, 2];
    let result = mul(&left, &right);
    dbg!(result);
}

// left: 32ビット幅の固定小数点Torus
// right: Z上の整数
// return: 32ビット幅の固定小数点Torus
#[cfg(feature = "fft")]
fn mul(left: &Vec<u32>, right: &Vec<u32>) -> Vec<u32> {
    let len = left.len();
    let w = Complex::new(-1.0, 0.0).powf(1. / len as f64);
    dbg!(w);

    let mut lfft: Vec<Complex<f64>> = left
        .iter()
        .enumerate()
        .map(|(i, l)| w.powf(i as f64 / 2.0) * Complex::new(*l as f64, 0.0))
        .collect();
    let mut lfft_out: Vec<Complex<f64>> = vec![Complex::zero(); len];
    let mut rfft: Vec<Complex<f64>> = right
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
    return result_fft_out
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            let tmp = (w.powf(i as f64 / -2.0) * c);
            dbg!(tmp);
            tmp.re as u32 / len as u32
        })
        .collect();
}
