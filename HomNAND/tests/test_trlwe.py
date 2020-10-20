from homnand import params
from hypothesis import given, settings, strategies as st
from typing import List

from homnand.trlwe import decrypt, encrypt, Ciphertext
from homnand import tlwe


@settings(max_examples=50, deadline=10000)
@given(st.lists(st.integers(min_value=0, max_value=1), min_size=params.N, max_size=params.N))
def test_trlwe(m: List[int]):
    ct, s, e = encrypt(m)
    m_hat = decrypt(ct, s)
    assert m == m_hat

@settings(max_examples=50, deadline=100000)
@given(st.lists(st.integers(min_value=0, max_value=1), min_size=params.N, max_size=params.N))
def test_sample_extract_index(m: List[int]):
    ct, s, e = encrypt(m)
    for k in range(m.__len__()):
        ct_tlwe = ct.sample_extract_index(k)
        m_hat = tlwe.decrypt(ct_tlwe, s)
        assert m[k] == m_hat
