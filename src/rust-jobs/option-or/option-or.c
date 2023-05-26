#include <stdio.h>

#include "seahorn/seahorn.h"
#include "inc/lib.h"

int sea_nd_int(void) {
    return 42;
}

int main() {
    int v = sea_nd_int();
    int w = sea_nd_int();

    int result = option_or(v, w);

    if ((v & 1) == 0) {
        sassert(result == v);
    } else {
        sassert(result == w);
    }

    return 42;
}
