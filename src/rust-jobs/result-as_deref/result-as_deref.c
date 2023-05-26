#include <stdio.h>

#include "seahorn/seahorn.h"
#include "inc/lib.h"

int sea_nd_int(void) {
    return 0;
}

int main() {
    int x = sea_nd_int();
    int res = as_deref(x);

    assume(x);
    sassert(res == x*2);
    
    return 0;
}