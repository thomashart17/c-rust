c_rust_llvm(smallvec-bound8 smallvec-bound8.c)

configure_file(sea.yaml sea.yaml @ONLY)
sea_discover_tests(smallvec-bound8)
