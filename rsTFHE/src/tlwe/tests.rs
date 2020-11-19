use quickcheck_macros::quickcheck;
use rand;

use super::*;

#[quickcheck]
fn test_tlwe(m: u32) -> bool {
    let m = m % 2;
    let n = params::n;
    let mu = params::mu;
    let s = gen_s(n);
    let (c, e) = encrypt_bin(m, &s, mu);
    let m_hat = decrypt(c.clone(), &s);
    if m != m_hat {
        dbg!(&c, &s, e, &c.a * &s, m, m_hat);
    }
    m == m_hat
}
