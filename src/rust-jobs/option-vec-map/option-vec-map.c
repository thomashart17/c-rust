#include <stdio.h>

#include "seahorn/seahorn.h"
#include "inc/lib.h"

uint8_t sea_nd_uint8_t(void) {
    return 42;
}

int main() {
    uint8_t v = sea_nd_uint8_t();
    unsigned int result = option_vec_map(v);

    sassert(result >= v*v);

    return 42;
}
