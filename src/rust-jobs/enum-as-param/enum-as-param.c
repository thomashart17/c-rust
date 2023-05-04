#include <stdio.h>

#include "seahorn/seahorn.h"

#include "inc/lib.h"

int main() {
    int enum_res = enum_param_test(KValTwo);
    printf("Enum result: %d\r\n", enum_res);

    sassert(enum_res == 102);

    return 42;
}