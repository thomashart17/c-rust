#include <stdio.h>

#include "seahorn/seahorn.h"

#include "../test-lib/inc/lib.h"

int main() {
    int res = add(5, 7);
    printf("Result: %d\r\n", res);

    sassert(res == 12);

    return 42;
}