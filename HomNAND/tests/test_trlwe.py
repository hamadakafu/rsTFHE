from homnand import params
from hypothesis import given, settings, strategies as st
from typing import List

from homnand.trlwe import decrypt, encrypt, Ciphertext, _dec_helper
from homnand import tlwe


# @settings(max_examples=50, deadline=10000)
# @given(st.lists(st.integers(min_value=0, max_value=1), min_size=params.N, max_size=params.N))
# def test_trlwe(m: List[int]):
#     ct, s, e = encrypt(m)
#     m_hat = decrypt(ct, s)
#     assert m == m_hat
#
# @settings(max_examples=50, deadline=100000)
# @given(st.lists(st.integers(min_value=0, max_value=1), min_size=params.N, max_size=params.N))
# def test_sample_extract_index(m: List[int]):
#     ct, s, e = encrypt(m)
#     for k in range(m.__len__()):
#         ct_tlwe = ct.sample_extract_index(k)
#         m_hat = tlwe.decrypt(ct_tlwe, s)
#         assert m[k] == m_hat

# @given(st.integers(min_value=0, max_value=(2**32) - 1))
def test_dec_helper():
    a = [1000000, 200000, 30000000]
    b = [1000000, 200000, 30000000]
    N = 3
    ans = _dec_helper(a, b, N=N)
    print(ans)
    for ll in range(params.l * 2):
        acc = 0
        for nn in range(N):
            i = ll if ll < params.l else ll - params.l
            acc += ans[ll][nn] * (64**(i + 1))
        print(acc)
