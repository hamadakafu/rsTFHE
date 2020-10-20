from __future__ import annotations
from homnand.util import sgn
from random import gauss
from typing import List, Tuple
import random

from homnand.torus import Torus01, TorusPoly, TorusVec
from homnand import params


def encrypt(m: List[int]) -> Tuple[Ciphertext, List[int], TorusPoly]:
    a = gen_a(params.N)
    s = gen_s(params.N)
    e = gen_e(params.alpha_bk, params.N)
    msg = TorusPoly([(Torus01(params.mu) * (2 * me - 1)).double for me in m])
    b = a * s + msg + e
    return Ciphertext(a, b), s, e


def decrypt(ct: Ciphertext, s: List[int]) -> List[int]:
    m_hat: List[int] = []
    for left, right in zip(ct.b.coef , (ct.a * s).coef):
        torus0505 = left.double - right.double
        torus0505 %= 1
        if torus0505 > 0.5:
            torus0505 -= 1
        m_hat.append((1 + sgn(torus0505)) // 2)

    return m_hat

class Ciphertext:
    def __init__(self, a: TorusPoly, b: TorusPoly) -> None:
        self.a = a
        self.b = b

    def __str__(self) -> str:
        return f"a: {self.a}, b: {self.b}"


def gen_s(n: int) -> List[int]:
    """
    係数が0,1の多項式を生成

    Parameters
    ----------
    n : int
        多項式の次元

    Returns
    -------
    List[int]
        [description]
    """
    return [random.randint(0, 1) for _ in range(n)]


def gen_a(n: int) -> TorusPoly:
    """[summary]

    Parameters
    ----------
    n : int
        ベクトルのサイズ

    Returns
    -------
    TorusVec
    """
    return TorusPoly([random.random() for _ in range(n)])


def gen_e(alpha: float, n: int) -> TorusPoly:
    return TorusPoly([random.gauss(0, alpha) for _ in range(n)])