use rsTFHE::spqlios::{
    fft, fft_table_get_buffer, ifft, ifft_table_get_buffer, new_fft_table, new_ifft_table,
};
use std::num::Wrapping;

// fn main() {
//     // 2 ** 32を法とした多項式環(x^n+1)
//
//     let nn: usize = 32;
//     let left: Vec<u32> = (0..nn as u32).collect();
//     let right: Vec<i32> = (-(nn as i32) / 2..nn as i32 / 2).collect();
//     assert_eq!(left.len(), right.len());
//
//     let mut coef = vec![Wrapping(0); nn * 2 - 1];
//     for (li, le) in left.iter().enumerate() {
//         for (ri, re) in right.iter().enumerate() {
//             if *re < 0 {
//                 coef[li + ri] -= Wrapping(*le * (-*re) as u32);
//             } else {
//                 coef[li + ri] += Wrapping(*le * *re as u32);
//             }
//         }
//     }
//     for i in (0..nn - 1).rev() {
//         let tmp = coef.pop().unwrap();
//         coef[i] -= tmp;
//     }
//     dbg!(left, right, coef);
// }

fn main() {
    let nn: usize = 32;
    let mut left: Vec<u32> = (0..nn as u32).collect();
    let mut right: Vec<i32> = (-(nn as i32) / 2..nn as i32 / 2).collect();
    let expect: Vec<u32> = {
        let mut coef = vec![Wrapping(0); nn * 2 - 1];
        for (li, le) in left.iter().enumerate() {
            for (ri, re) in right.iter().enumerate() {
                if *re < 0 {
                    coef[li + ri] -= Wrapping(*le * (-*re) as u32);
                } else {
                    coef[li + ri] += Wrapping(*le * *re as u32);
                }
            }
        }
        for i in (0..nn - 1).rev() {
            let tmp = coef.pop().unwrap();
            coef[i] -= tmp;
        }
        dbg!(&left, &right, &coef);
        coef.into_iter().map(|a| a.0).collect()
    };

    let mut left_tmp: Vec<f64> = (0..nn).map(|_| 0.0).collect();
    let mut right_tmp: Vec<f64> = (0..nn).map(|_| 0.0).collect();
    let mut real_tmp: Vec<f64> = (0..nn).map(|_| 0.0).collect();
    let mut real: Vec<u32> = (0..nn as u32).collect();
    unsafe {
        let fft_table = new_fft_table(nn as i32);
        let ifft_table = new_ifft_table(nn as i32);

        let buf_fft = fft_table_get_buffer(fft_table);
        let buf_ifft = ifft_table_get_buffer(ifft_table);

        println!("before fft");

        for i in 0..nn {
            let p = buf_fft.offset(i as isize);
            *p = left[i as usize] as f64;
        }
        fft(fft_table, buf_fft);
        for i in 0..nn {
            let p = buf_fft.offset(i as isize);
            left_tmp[i as usize] = *p;
        }
        for i in 0..nn {
            let p = buf_fft.offset(i as isize);
            *p = right[i as usize] as f64;
        }
        fft(fft_table, buf_fft);
        for i in 0..nn {
            let p = buf_fft.offset(i as isize);
            right_tmp[i as usize] = *p;
        }

        for i in 0..nn {
            let p = buf_ifft.offset(i as isize);
            *p = left_tmp[i] * right_tmp[i];
        }
        ifft(ifft_table, buf_ifft);
        for i in 0..nn {
            let p = buf_ifft.offset(i as isize);
            real_tmp[i] = *p;
        }
    }
    for i in 0..nn {
        let mut tmp = real_tmp[i] as i128;
        tmp /= nn as i128 / 2;
        tmp %= u32::MAX as i128 + 1;
        if tmp < 0 {
            tmp += u32::MAX as i128 + 1;
        }
        assert!(0 <= tmp && tmp < u32::MAX as i128 + 1);
        real[i] = tmp as u32;
    }
    dbg!(real_tmp, real, expect);
    // assert_eq!(expect, real);
}
