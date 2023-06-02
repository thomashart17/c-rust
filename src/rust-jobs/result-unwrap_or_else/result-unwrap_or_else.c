#include <stdio.h>

#include "seahorn/seahorn.h"
#include "inc/lib.h"

int sea_nd_int() {
    return 0;
}

int main() {
    int x = sea_nd_int();
    int y = sea_nd_int();
    int res = unwrap_or_else(x, y);

    assume(y <= 0);
    sassert(res == -1);


    assume(x <= 0);
    assume(y > 0);
    sassert(res == -1);

    assume(x >= 0);
    assume(y > 0);
    sassert(res > x/y);

    return 42;
}