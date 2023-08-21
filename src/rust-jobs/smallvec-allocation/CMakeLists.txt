c_rust_llvm(smallvec-allocation smallvec-allocation.c)

configure_file(sea.yaml sea.yaml @ONLY)
sea_discover_tests(smallvec-allocation)
