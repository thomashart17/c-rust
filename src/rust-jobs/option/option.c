#include <stdio.h>

#include "seahorn/seahorn.h"

#include "inc/lib.h"

int main() {
    int option_res = option_test(10);
    printf("Val: %d\r\n", option_res);

    sassert(option_res == 20);

    return 42;
}