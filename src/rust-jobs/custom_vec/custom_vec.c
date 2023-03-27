#include <stdio.h>

#include "seahorn/seahorn.h"

#include "../test-lib/inc/lib.h"

int main() {

    bool custom_vec_res = true;
        
    custom_vec_res &= create_push_pop();
    printf("After test 1: %d\r\n", custom_vec_res);


    sassert(custom_vec_res == 1);

    return 42;
}