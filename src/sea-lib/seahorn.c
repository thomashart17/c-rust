/**
 * \brief Dummy implementation of SeaHorn verification builtins
 *
 * Dummy implementation is necessary to build Rust project with LTO.
 * The bodies of these functions are removed by SeaHorn before any 
 * optimizataion and verification takes place 
 */
#include "seahorn/seahorn.h"

void __VERIFIER_error (void) {
    return;
}

void __VERIFIER_assume (int i) {
    return;
}

void __SEA_assume(bool b) {
    return;
}
