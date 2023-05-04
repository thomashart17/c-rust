#include <stdio.h>

#include "seahorn/seahorn.h"

#include "inc/lib.h"

int main() {
    int n = 0;
    modify_ptr(&n);
    printf("N: %d\r\n", n);

    sassert(n == 3);

    return 42;
}