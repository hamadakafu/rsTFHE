from homnand import params
from hypothesis import given, settings, strategies as st
from typing import List

from homnand.trlwe import decrypt, encrypt, Ciphertext


@settings(max_examples=50, deadline=10000)
@given(st.lists(st.integers(min_value=0, max_value=1), min_size=params.N, max_size=params.N))
def test_trlwe(m: List[int]):
    ct, s, e = encrypt(m)
    m_hat = decrypt(ct, s)
    assert m == m_hat
