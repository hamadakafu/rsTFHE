from homnand import params
import pytest
import random

from homnand.torus import Torus01, TorusVec, TorusPoly
from hypothesis import given, note, strategies as st


@given(st.integers(min_value=-1000, max_value=1000))
def test_torus_eq(d):
    t = Torus01(d)
    assert t == Torus01(d % 1)


@given(
    st.floats(min_value=-1000, max_value=1000),
    st.floats(min_value=-1000, max_value=1000),
)
def test_torus_add(d1, d2):
    left = Torus01(d1)
    right = Torus01(d2)
    assert (left + right) == Torus01(d1 + d2)


# @given(
#     st.floats(min_value=-1000, max_value=1000),
#     st.floats(min_value=-1000, max_value=1000),
# )
# def test_torus_sub(d1, d2):
#     left = Torus01(d1)
#     right = Torus01(d2)
#     assert (left - right) == Torus01(d1 - d2)


@given(
    st.floats(min_value=-1000, max_value=1000),
    st.integers(min_value=-1000, max_value=1000),
)
def test_torus_mul(d1, i):
    left = Torus01(d1)
    assert (left * i) == Torus01((d1 % 1) * i)


@given(
    st.lists(
        st.floats(min_value=-1000, max_value=1000),
        min_size=10,
        max_size=10,
    ),
    st.lists(
        st.integers(min_value=-1000, max_value=1000),
        min_size=10,
        max_size=10,
    ),
)
def test_torus_vec_mul(lf, li):
    tv = TorusVec(lf)
    acc = 0
    for (f, i) in zip(lf, li):
        acc += ((f % 1) * i)
    expect = Torus01(acc)
    ans = tv * li
    assert  ans == expect

@given(
    st.lists(
        st.floats(min_value=0, max_value=1),
        min_size=10,
        max_size=10,
        unique=True,
    ),
    st.lists(
        st.integers(min_value=-1000, max_value=1000),
        min_size=10,
        max_size=10,
        unique=True,
    ),
)
def test_torus_poly_mul(lf, li):
    tp = TorusPoly(lf)
    tp * li