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