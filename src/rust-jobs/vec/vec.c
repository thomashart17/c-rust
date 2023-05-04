#include <stdio.h>

#include "seahorn/seahorn.h"

#include "inc/lib.h"

int main() {
    int vec_len = vec_test();
    printf("Vec len: %d\r\n", vec_len);

    sassert(vec_len == 4);

    return 42;
}