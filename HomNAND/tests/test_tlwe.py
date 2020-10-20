from hypothesis import given, settings, strategies as st

import random
from homnand.tlwe import decrypt, encrypt


@settings(max_examples=50, deadline=10000)
@given(st.integers(min_value=0, max_value=1))
def test_tlwe(m):
        ct, s, e = encrypt(m)
        m_hat = decrypt(ct, s)
        assert m == m_hat

@settings(max_examples=50, deadline=10000)
@given(st.lists(st.integers(min_value=0, max_value=1), max_size=5))
def test_tlwe_many(ms):
    for m in ms:
        ct, s, e = encrypt(m)
        m_hat = decrypt(ct, s)
        assert m == m_hat
            # print(f'ct: {ct}, s: {s}, m: {m}, m_hat: {m_hat}')
