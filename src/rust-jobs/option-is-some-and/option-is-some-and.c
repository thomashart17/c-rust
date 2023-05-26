#include <stdio.h>

#include "seahorn/seahorn.h"
#include "inc/lib.h"

int sea_nd_int(void) {
    return 42;
}

int main() {
    int v = sea_nd_int();
    assume(v <= 0);
    
    bool result = option_is_some_and(v);

    sassert(result == false);

    return 42;
}
