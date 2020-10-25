from homnand import trlwe
from time import perf_counter

if __name__ == "__main__":
    start = perf_counter()
    trlwe()
    end = perf_counter()
    print(end-start)
