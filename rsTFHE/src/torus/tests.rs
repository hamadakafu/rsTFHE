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
fn test_vec_poly(left: Vec<Wrapping<u32>>, right: Vec<i64>) -> bool {
    if left.len() != right.len() {
        return true;
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
