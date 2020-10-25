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
    let (c, s, e) = encrypt(m.clone());
    let m_hat = decrypt(c, s);
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

    let (c, s, e) = encrypt(m.clone());
    let tlwe = c.sample_extract_index(idx);
    let m_idx_hat = tlwe::decrypt(tlwe, s);

    if m[idx] as u32 != m_idx_hat {
        dbg!(idx, m[idx], m_idx_hat, &m);
    }
    m[idx] as u32 == m_idx_hat
}
