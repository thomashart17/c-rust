/**
 * \brief Dummy implementation of SeaHorn verification builtins
 *
 * Dummy implementation is necessary to build Rust project with LTO.
 * The bodies of these functions are removed by SeaHorn before any
 * optimizataion and verification takes place
 */
#include "seahorn/seahorn.h"
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h> // for free

void __VERIFIER_error(void) { return; }

void __VERIFIER_assume(int i) { return; }

void __SEA_assume(bool b) { return; }

int32_t sea_nd_i32(void) { return 0; }

bool sea_nd_bool(void) { return true; }

void *sea_realloc(void *ptr, size_t sz) {
  if (ptr)
    free(ptr);
  return malloc(sz);
}

void *realloc(void *ptr, size_t sz) { return sea_realloc(ptr, sz); }
