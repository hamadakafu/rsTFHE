#!/bin/bash
set -exuo pipefail

CFLAGS="-g3 -O0 -Wall -DNDEBUG"
CFLAGS="-g0 -O2 -DNDEBUG"

g++ -c $CFLAGS -o spqlios-bench.o spqlios-bench.cpp
g++ -c $CFLAGS -o my-test.o my-test.cpp
g++ -c $CFLAGS -o spqlios-fft-impl.o spqlios-fft-impl.cpp
g++ -c $CFLAGS -o spqlios-fft.o spqlios-fft.s
g++ -c $CFLAGS -o spqlios-ifft.o spqlios-ifft.s
g++ $CFLAGS -o spqlios-bench spqlios-bench.o spqlios-fft-impl.o spqlios-fft.o spqlios-ifft.o
g++ $CFLAGS -o my-test my-test.o spqlios-fft-impl.o spqlios-fft.o spqlios-ifft.o
