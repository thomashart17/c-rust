#include <stdio.h>

#include "seahorn/seahorn.h"

#include "inc/lib.h"

int sea_nd_int(void) {
    return 42;
}

int main() {
    int v = sea_nd_int();
    assume(v >= 1);
    int res = add(v, 7);
    printf("Result: %d\r\n", res);

    if (v > 0) {
        sassert(res > 7);
    } else {
        sassert(res <= 7);
    }

    return 42;
}