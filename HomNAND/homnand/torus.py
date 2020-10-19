from __future__ import annotations
import random
from typing import List
from homnand import torus
from homnand import params

class Torus01:
    """
    Torus(円周群)
    範囲: [0, 1)
    """

    def __init__(self, d: float):
        tmp = d % 1
        self.double = tmp + 1 if tmp < 0 else tmp
        self.fix = int(self.double * (2 ** params.w))

    def __str__(self) -> str:
        return f'T(double: {self.double}, fix: {bin(self.fix)})'
    def __repr__(self) -> str:
        return f'T(double: {self.double}, fix: {self.fix})'

    @staticmethod
    def modular_normal(alpha: float) -> Torus01:
        """
        モジュラー正規分布
        alpha: 標準偏差
        """
        # TODO: 安全じゃないので修正する
        a = random.gauss(0, alpha)
        return Torus01(a % 1)

    def __add__(self, other: Torus01) -> Torus01:
        return Torus01(self.double + other.double)
        # return Torus01(((self.fix + other.fix) & (2**params.w)) / dou(2 ** params.w))

    # def __sub__(self, other: Torus01):
    #     return Torus01(self.double - other.double)
    #     return Torus01(self.fix ^ other.fix / (2 ** params.w))

    def __mul__(self, other: int):
        """
        整数となら積が定義できる
        """
        return Torus01(self.double * other)

    def __eq__(self, other: Torus01):
        return self.fix == other.fix


class TorusVec:
    """
    要素がTorusのベクトル
    """

    @staticmethod
    def sample(size: int) -> TorusVec:
        elm = []
        for _ in range(size):
            elm.append(random.uniform(0, 1))
        return TorusVec(elm)

    def __init__(self, elements: List[float]):
        ts: List[Torus01] = []
        for e in elements:
            ts.append(Torus01(e))
        self.elements = ts

    def __mul__(self, other: List[int]) -> Torus01:
        """
        整数ベクトルとの内積
        """
        assert(len(self.elements) == len(other))
        acc = Torus01(0)
        for (l, r) in zip(self.elements, other):
            acc += l * r
        return acc

    def __str__(self) -> str:
        return f'TorusVec: {self.elements}'
    def __repr__(self) -> str:
        return f'TorusVec: {self.elements}'


class TorusPoly:
    """
    係数がTorusの多項式
    """

    def __init__(self, coef: List[Torus01]):
        self.coef = coef
