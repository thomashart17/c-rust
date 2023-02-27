#include <stdio.h>

#include "seahorn/seahorn.h"

#include "../test-lib/inc/lib.h"

int main() {
    int vec_len = vec_test();
    printf("Vec len: %d\r\n", vec_len);

    int n = 0;
    modify_ptr(&n);
    printf("N: %d\r\n", n);

    sassert(vec_len == 5);
    sassert(n == 2);

    return 42;
}