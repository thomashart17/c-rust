#include <stdio.h>

#include "inc/lib.h"
#include "seahorn/seahorn.h"

int sea_nd_int() { return 0; }

int main() {
  int x = sea_nd_int();
  int y = sea_nd_int();
  int res = divide_and_multiply(x, y);

  assume(x > 0);
  assume(y <= 0);
  sassert(res == -1);

  /* assume(x <= 0); */
  /* assume(y > 0); */
  /* sassert(res == -1); */

  /* assume(x >= 0); */
  /* assume(y > 0); */
  /* sassert(res > 0); */

  return 42;
}

// int v = sea_nd_int();
// assume(v >= 1);
// int res = add(v, 7);
// printf("Result: %d\r\n", res);

// sassert(res >= 7);

// return 42;
