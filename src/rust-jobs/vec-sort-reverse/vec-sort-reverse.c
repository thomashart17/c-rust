#include <stdio.h>

#include "seahorn/seahorn.h"
#include "inc/lib.h"

int sea_nd_int(void) {
    return 42;
}

int main() {
    int x = sea_nd_int();
    int y = sea_nd_int();
    int z = sea_nd_int();

    assume((x <= y) && (x <= z));

    int result = vec_sort_reverse(x, y, z);

    sassert(result >= x);

    return 42;
}
