#include <stdio.h>

#include "seahorn/seahorn.h"

#include "inc/lib.h"\

int sea_nd_int(void) {
    return 42;
}

int main() {
    int enum_res = enum_param_test(KValTwo);
    int v = sea_nd_int();
    assume(v == 102);

    printf("Enum result: %d\r\n", enum_res);

    sassert(enum_res == v);

    return 42;
}
