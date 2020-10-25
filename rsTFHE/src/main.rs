use rsTFHE::torus::Torus01;
use std::num::Wrapping;
use rand_distr::{Normal, Distribution};
use rand::Rng;

fn main() {
    let left = -38.03969031931551;
    let right = -99.27130602164213;
    let lt = Torus01::new_with_float(left);
    // let rt = Torus01::new_with_float(right);
    // dbg!(lt + rt, Torus01::new_with_float(left + right));
    // let normal = Normal::new(0., 0.3).unwrap();
    // dbg!(Torus01::new_with_float(normal.sample(&mut rand::thread_rng())));
    // let normal = Normal::new(0., 0.3).unwrap();
    // dbg!(Torus01::new_with_float(normal.sample(&mut rand::thread_rng())));
    dbg!(lt, lt * -3);
    dbg!(lt, lt * 3);

    let mut rng = rand::thread_rng();
    let hoge: Vec<u32> = (0..100).map(|_| rng.gen_range(1, 10)).collect();
    dbg!(hoge);
}
