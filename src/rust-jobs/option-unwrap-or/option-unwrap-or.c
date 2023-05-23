#include <stdio.h>

#include "seahorn/seahorn.h"
#include "inc/lib.h"

int sea_nd_int(void) {
    return 42;
}

int main() {
    int v = sea_nd_int();
    assume(v % 2 == 1);

    int result = option_unwrap_or(v);

    sassert(result == 0);

    return 42;
}
