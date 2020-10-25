#include <stdio.h>
#include <sys/_types/_u_int32_t.h>

int main() {

  unsigned int a = 10000;
  unsigned int b = 10000000;
  unsigned int c = a * b;
  printf("%d\n", c);
}
