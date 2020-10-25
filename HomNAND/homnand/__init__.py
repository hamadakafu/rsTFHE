from homnand.torus import Torus01
from homnand.trlwe import encrypt,decrypt
from homnand import params
import random
__version__ = '0.1.0'

def main():
    m = [random.randint(0, 1) for _ in range(params.N)]
    ct, s, e = encrypt(m)
    m_hat = decrypt(ct, s)
    assert m == m_hat

def trlwe():
    m = [random.randint(0, 1) for _ in range(params.N)]
    ct, s, e = encrypt(m)
    m_hat = decrypt(ct, s)
    assert m == m_hat

def playground():
    import numpy as np
    # unsigned int a = 10000;
    # unsigned int b = 10000000;

    print(np.array([10000], dtype='uint32') * np.array([10000000], dtype='uint32'))
    a = np.array(240000, dtype='uint32')
    print(a * a)
    print(type(a >> 3))

    # aa = (np.array(a, dtype='uint32') >> (32 - params.Bgbit * idx)) & (params.Bg - 1)
    print('ff', (np.array(1000000000, dtype='uint32') >> (32 - params.Bgbit * 1)) & (params.Bg-1))


    a = 0.3
    a_list = []
    bgbit = 6
    bg = 2 ** 6
    l = 3
    for i in range(1, l + 1):
        print(np.array(a * ((bg / 2) ** i), dtype='uint32'))

