#include <stdint.h>
#include <cmath>

#include <string>
#include <cassert>
#include <vector>
#include <iostream>
#include <cstdlib>
#include <complex>

#include "spqlios-fft.h"

using namespace std;

int32_t main(int32_t argc, char** argv) {
    static const int32_t nn = 32;
    void* tables = new_fft_table(nn);
    void* itables = new_ifft_table(nn);
    double* buf_fft = fft_table_get_buffer(tables);
    double* buf_ifft = ifft_table_get_buffer(itables);
    double* a = new double[nn];
    double* a2 = new double[nn];
    double* b = new double[nn];
    for (int32_t i=0; i<nn; i++) a[i]=i;
    for (int32_t i=0; i<nn; i++) a2[i]=i;
    for (int32_t i=0; i<nn; i++) b[i]=a[i];

    printf("before fft\n");

    for (int32_t i=0; i<nn; i++) printf("%lf ", a[i]);
    printf("\n");

    for (int32_t i=0; i<nn; i++) buf_fft[i]=a[i];
    fft_model(tables);
    for (int32_t i=0; i<nn; i++) a[i]=buf_fft[i];

    for (int32_t i=0; i<nn; i++) buf_fft[i]=a2[i];
    fft(tables,buf_fft);
    for (int32_t i=0; i<nn; i++) a2[i]=buf_fft[i];

    printf("after fft\n");
    for (int32_t i=0; i<nn; i++) printf("a: %lf, a2: %lf\n", a[i], a2[i]);
    printf("\n");
    printf("before ifft\n");

    for (int32_t i=0; i<nn; i++) buf_ifft[i]=a[i];
    ifft_model(itables);
    for (int32_t i=0; i<nn; i++) a[i]=buf_ifft[i];

    for (int32_t i=0; i<nn; i++) buf_ifft[i]=a2[i];
    ifft(itables,buf_ifft);
    for (int32_t i=0; i<nn; i++) a2[i]=buf_ifft[i];

    printf("after ifft\n");
    for (int32_t i=0; i<nn; i++) printf("%lf ", a2[i]);
    printf("\n");
}
