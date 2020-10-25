from __future__ import annotations
from typing import List, Tuple
import random
import numpy as np

from homnand.torus import Torus01, TorusPoly, TorusVec
from homnand.util import sgn
from homnand import params, tlwe


def encrypt(m: List[int]) -> Tuple[Ciphertext, List[int], TorusPoly]:
    a = gen_a(params.N)
    s = gen_s(params.N)
    e = gen_e(params.alpha_bk, params.N)
    msg = TorusPoly([(Torus01(params.mu) * (2 * me - 1)).double for me in m])
    b = a * s + msg + e
    return Ciphertext(a, b), s, e


def decrypt(ct: Ciphertext, s: List[int]) -> List[int]:
    m_hat: List[int] = []
    for left, right in zip(ct.b.coef, (ct.a * s).coef):
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

    def sample_extract_index(self, k: int) -> tlwe.Ciphertext:
        """
        k番目のTLWE問題を作成する

        Parameters
        ----------
        k : int
            k番目のTLWE

        Returns
        -------
        tlwe.Ciphertext
            [description]
        """
        a = [ ]
        N = self.a.coef.__len__()
        for idx in range(self.a.coef.__len__()):
            if idx <= k:
                a.append(self.a.coef[k - idx].double)
            else:
                a.append(-self.a.coef[N + k - idx].double)

        assert a.__len__() == self.a.coef.__len__()
        return tlwe.Ciphertext(TorusVec(a), self.b.coef[k])

    def decomposition(self) -> List[List[int]]:
        ab = _dec_helper(list(map(lambda t: t.fix, self.a.coef)), list(map(lambda t: t.fix, self.b.coef)))
        return ab

def _dec_helper(a: List[int], b: List[int], N: int = params.N, l: int = params.l, Bg: int = params.Bg, Bgbit: int = params.Bgbit) -> List[List[int]]:
    ab = [[np.array(0, dtype='uint32') for _ in range(N)] for _ in range(l * 2)]
    for i in range(0, l):
        for j in range(N):
            aij = _dec_one_helper(a[j], i + 1)
            print('aij', aij)
            tmp = aij +  np.array(Bg/2, dtype='uint32')
            if i > 0:
                # Bgbit分から溢れた分を上の階層に足す
                ab[i - 1][j] += np.array(tmp >> Bgbit, dtype='uint32')
            ab[i][j] = np.array((tmp & np.array((Bg - 1), dtype='uint32')) - Bg / 2, dtype='uint32')

    for i in range(0, l):
        for j in range(N):
            bij = _dec_one_helper(b[j], i + 1)
            tmp = bij +  np.array(Bg/2, dtype='uint32')
            if i > 0:
                # Bgbit分から溢れた分を上の階層に足す
                ab[i - 1 + l][j] += np.array(tmp >> Bgbit, dtype='uint32')
            ab[i + l][j] = np.array((tmp & (Bg - 1)) - Bg / 2, dtype='uint32')
    return ab

def _dec_one_helper(a: int, idx: int) -> int:
    """
    decompositionのhelper

    Parameters
    ----------
    a : int
        decompostionされるターゲット
    idx : int
        何番目のbit達を取り出すか1始まり
    Returns
    -------
    int
        [description]
    """
    aa = (np.array(a, dtype='uint32') >> (32 - params.Bgbit * idx)) & (params.Bg - 1)
    return aa


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