#include <stdio.h>

#include "seahorn/seahorn.h"
#include "inc/lib.h"

int sea_nd_int(void) {
    return 0;
}

int main() {
    int x = sea_nd_int();
    int res = iter_mut(x);

    sassert(res == ((x < 0) ? 2*x : x));
    
    return 0;
}