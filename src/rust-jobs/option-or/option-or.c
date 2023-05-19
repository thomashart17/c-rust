#include <stdio.h>

#include "seahorn/seahorn.h"
#include "inc/lib.h"

int sea_nd_int(void) {
    return 42;
}

int main() {
    int v = sea_nd_int();
    assume(v % 2 == 1);

    int result = option_or(v, 2);

    sassert(result == 2);

    return 42;
}
