c_rust_llvm(smallvec-insert-optimization smallvec-insert-optimization.c CUSTOM_PANIC_NO_STD)

configure_file(sea.yaml sea.yaml @ONLY)
sea_add_unsat_test(smallvec-insert-optimization)
