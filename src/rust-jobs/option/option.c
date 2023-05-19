#include <stdio.h>

#include "seahorn/seahorn.h"

#include "inc/lib.h"

int sea_nd_int(void) {
    return 42;
}

int main() {
    int v = sea_nd_int();

    assume(v % 2 == 0);
    assume(v > 0);

    int option_res = option_test(v);

    printf("Val: %d\r\n", option_res);

    sassert(option_res > v);

    return 42;
}