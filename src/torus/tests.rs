use quickcheck_macros::quickcheck;
use rand;

use super::*;

#[ignore]
#[quickcheck]
fn test_new_with_float(float: f64) -> bool {
    let t = Torus01::new_with_float(float);
    t.float == float
}

#[quickcheck]
fn test_new_with_fix(fix: Wrapping<u32>) -> bool {
    let t = Torus01::new_with_fix(fix);
    t.fix == fix
}

#[quickcheck]
fn test_add_fix(left: Wrapping<u32>, right: Wrapping<u32>) -> bool {
    let lt = Torus01::new_with_fix(left);
    let rt = Torus01::new_with_fix(right);
    lt + rt == Torus01::new_with_fix(left + right)
}

#[ignore]
#[quickcheck]
fn test_add_float(left: f64, right: f64) -> bool {
    let lt = Torus01::new_with_float(left);
    let rt = Torus01::new_with_float(right);
    lt + rt == Torus01::new_with_float(left + right)
}

// TODO: 確かなテスト方法がわからないの
// #[test]
// fn test_mul_fix() {
//     let table = [
//         (Wrapping(4781023), 3),
//         (Wrapping(4781023), -3),
//         (Wrapping(47019247), 3),
//         (Wrapping(857031541), -3),
//     ];
//     for (left, right) in table.iter() {
//         let lt = Torus01::new_with_fix(*left);
//             assert_eq!(
//                 Torus01::new_with_fix(Wrapping(u32::MAX)),
//                 Torus01::new_with_fix(*left) * *right + Torus01::new_with_fix(*left) * -right,
//             );
//     }
// }

#[quickcheck]
fn test_vec_poly(mut left: Vec<Wrapping<u32>>, mut right: Vec<i64>) -> bool {
    if left.len() == 0 || right.len() == 0 {
        return true;
    }
    if left.len() > right.len() {
        right = right.into_iter().cycle().take(left.len()).collect();
    } else if left.len() < right.len() {
        left = left.into_iter().cycle().take(right.len()).collect();
    }
    let ltv = Torus01Vec::new_with_fix(left.clone());
    let real = &ltv * &right;
    let mut acc = Wrapping(0);
    for (l, r) in left.clone().into_iter().zip(right.iter()) {
        if *r < 0 {
            acc -= l * Wrapping(r.abs() as u32);
        } else {
            acc += l * Wrapping(*r as u32);
        }
    }
    let expect = Torus01::new_with_fix(acc);
    if real != expect {
        dbg!(left, right, real, expect, acc);
    }
    real == expect
}

#[test]
// TODO: なんか数字が一個ずれるバグがある
fn test_poly_mul_fft_fjiaof() {
    let mut left: Vec<Wrapping<u32>> = vec![Wrapping(u32::MAX - 1), Wrapping(u32::MAX - 2)];
    let left = Torus01Poly::new_with_fix(left);
    let mut right: Vec<i64> = vec![3, 4];
    let expect: Vec<u32> = vec![6, 4294967279];
    debug_assert_eq!(
        &left * &right,
        Torus01Poly::new_with_fix(expect.into_iter().map(|i| Wrapping(i)).collect())
    );
}

#[quickcheck]
fn test_poly_mul_fft(mut left: Vec<Wrapping<u32>>, mut right: Vec<i64>) -> bool {
    if left.len() == 0 || right.len() == 0 {
        return true;
    }
    let len = usize::max(
        left.len().next_power_of_two(),
        right.len().next_power_of_two(),
    );
    left = left.into_iter().cycle().take(len).collect();
    right = right.into_iter().cycle().take(len).collect();
    let mut left = Torus01Poly::new_with_fix(left);
    left = Torus01Poly::new_with_torus(left.coef.iter().map(|c| *c * 1).collect());

    let mut expect = vec![Torus01::new_with_fix(Wrapping(0)); left.coef.len() * 2 - 1];
    // TODO: fft使う
    for (li, le) in left.coef.iter().enumerate() {
        for (ri, re) in right.iter().enumerate() {
            expect[li + ri] += *le * *re;
        }
    }
    for i in (0..left.coef.len() - 1).rev() {
        let tmp = expect.pop().unwrap();
        expect[i] -= tmp;
    }

    let real = &left * &right;
    let result = real
        .coef
        .iter()
        .zip(expect.iter())
        .all(|(r, e)| (*r - *e).fix.0 < 5 || (*e - *r).fix.0 < 5);
    if !result {
        dbg!(left, right, &real.coef, &expect);
    }
    return result;
}
