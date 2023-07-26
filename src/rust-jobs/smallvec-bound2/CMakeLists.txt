c_rust_llvm(smallvec-bound2 smallvec-bound2.c)

configure_file(sea.yaml sea.yaml @ONLY)
sea_discover_tests(smallvec-bound2)
