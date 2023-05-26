#include <stdio.h>

#include "seahorn/seahorn.h"
#include "inc/lib.h"

int main() {
    int result = string_parse();

    sassert(result == 42);

    return 42;
}
