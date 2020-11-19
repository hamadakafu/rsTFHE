use quickcheck_macros::quickcheck;

use super::*;
use crate::params;
use crate::tlwe;

#[quickcheck]
fn test_trlwe(m: Vec<u8>) -> bool {
    if m.len() == 0 {
        return true;
    }
    let m: Vec<u8> = m
        .into_iter()
        .cycle()
        .take(params::N)
        .map(|e| e % 2)
        .collect();
    // dbg!("do test", &m);
    let s = gen_s(params::N);
    let (c, e) = encrypt_bin(m.clone(), &s);
    let m_hat = decrypt_bin(c, &s);
    m == m_hat
}

#[quickcheck]
fn test_sample_extract_index(m: Vec<u8>, idx: usize) -> bool {
    if m.len() == 0 {
        return true;
    }
    let m: Vec<u8> = m
        .into_iter()
        .cycle()
        .take(params::N)
        .map(|e| e % 2)
        .collect();
    let idx = idx % params::N;

    let s = gen_s(params::N);
    let (c, e) = encrypt_bin(m.clone(), &s);
    let tlwe = c.sample_extract_index(idx);
    let m_idx_hat = tlwe::decrypt(tlwe, &s);

    if m[idx] as u32 != m_idx_hat {
        dbg!(idx, m[idx], m_idx_hat, &m);
    }
    m[idx] as u32 == m_idx_hat
}

#[quickcheck]
fn test_trlwe_add_sub(m: Vec<u8>) -> bool {
    if m.len() == 0 {
        return true;
    }
    let m: Vec<u8> = m
        .into_iter()
        .cycle()
        .take(params::N)
        .map(|e| e % 2)
        .collect();

    let N = params::N;
    let s = gen_s(N);
    let zero = vec![0; N];
    let (cm, _) = encrypt_bin(m.clone(), &s);
    let (czero, _) = encrypt_torus(zero, &s);

    let cmcm = &cm - &czero;
    decrypt_bin(cmcm, &s) == m
}

