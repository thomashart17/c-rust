#include <stdio.h>

#include "seahorn/seahorn.h"
#include "inc/lib.h"

// extern void sea_printf(const char *format, ...);
// void sea_printf(const char *format, ...) {
// }

int sea_nd_int(void) {
    return 42;
}

int main() {
    int v = sea_nd_int();
    assume(v > 0);

    int result = option_and_then(v);

    sea_printf("Result: result, v", result, v);

    if ((v & 1) == 0) {
        sassert(result > v);
    } else {
        sassert(result == 0);
    }

    return 42;
}
