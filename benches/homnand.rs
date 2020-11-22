use criterion::black_box;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

use rsTFHE::{params, prelude::*};

fn nand(b1: bool, b2: bool) -> bool {
    // 暗号化
    let n = params::n;
    let mu_bit = params::mu_bit;

    // blind rotate
    let N = params::N;
    let N_bit = params::N_bit;
    let l = params::l;
    let bg = params::bg;
    let bgbit = params::bgbit;
    let t = params::t;
    let basebit = params::basebit;

    let s_tlwe = gen_tlwe_key(n);

    let (t1, _) = encrypt(b1 as u32, &s_tlwe, 1.0 / 2.0_f64.powi(mu_bit as i32));
    let (t2, _) = encrypt(b2 as u32, &s_tlwe, 1.0 / 2.0_f64.powi(mu_bit as i32));

    let s_trlwe = gen_trlwe_key(N);

    let trgsws = encrypt_tlwe_s(&s_tlwe, &s_trlwe, l, bg, bgbit);

    let ks = gen_ks(&s_trlwe, &s_tlwe, t, basebit);

    let out = homnand(t1, t2, trgsws, ks);

    // decrypt
    match decrypt(out, &s_tlwe) {
        0 => false,
        1 => true,
        _ => panic!("wtfjiefjafjoeajf"),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("nand(false, true)", |b| {
        b.iter(|| debug_assert_eq!(true, nand(false, true)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
