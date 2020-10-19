from hypothesis import given, note, strategies as st

from homnand.tlwe import decrypt, encrypt


@given(st.integers(min_value=0, max_value=1))
def test_tlwe(m):
        ct, s = encrypt(m)
        m_hat = decrypt(ct, s)
        assert m == m_hat

@given(st.lists(st.integers(min_value=0, max_value=1), max_size=2))
def test_tlwe_many(ms):
    for m in ms:
        ct, s = encrypt(m)
        m_hat = decrypt(ct, s)
        assert m == m_hat
            # print(f'ct: {ct}, s: {s}, m: {m}, m_hat: {m_hat}')
