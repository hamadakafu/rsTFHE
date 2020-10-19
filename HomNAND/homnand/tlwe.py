from __future__ import annotations

from random import gauss
from typing import List, Tuple
from homnand.torus import Torus01, TorusVec
from homnand import params
import random


def encrypt(m: int) -> Tuple[Ciphertext, List[int]]:
    # FIXME: メッセージは0か1のみだがTorusに拡大してもいい(0, \mu)
    assert m == 0 or m == 1
    a = gen_a(params.n)
    s = gen_s(params.n)
    e = gen_e(params.alpha)
    b = a * s + Torus01(params.mu * (2 * m  - 1)) + e
    return Ciphertext(a, b), s

def decrypt(ct: Ciphertext, s: List[int]) -> int:
    torus0505 = ct.b.double
    torus0505 -= (ct.a * s).double

    m_hat = int((1+ sgn(torus0505)) / 2)
    assert(m_hat == 0 or m_hat == 1)
    return m_hat

def sgn(x: float) -> int:
    """[summary]
    符号関数
    x > 0  =>  1
    x == 0 =>  0
    x < 0  => -1
    Parameters
    ----------
    x : float

    Returns
    -------
    int
    """
    if x > 0:
        return 1
    elif x == 0:
        print('wtfffff'*100)
        return 0
    else:
        return -1

class Ciphertext:
    def __init__(self, a: TorusVec, b: Torus01):
        self.a = a
        self.b = b

    def __str__(self) -> str:
        return f'a: {self.a}, b: {self.b}'


def gen_s(n: int) -> List[int]:
    ans = []
    for _ in range(n):
        # FIXME: randomを使うのをやめる安全じゃない
        ans.append(random.randint(0, 1))
    return ans


def gen_a(n: int) -> TorusVec:
    """[summary]

    Parameters
    ----------
    n : int
        多項式の次数

    Returns
    -------
    TorusVec
    """
    return TorusVec.sample(n)


def gen_e(alpha: float) -> Torus01:
    g = random.gauss(0, alpha)
    # アウト g = 1/8
    # アウト g = -1/8
    e = Torus01(g)
    # print(f'g: {g}, e: {e}')
    return e