c_rust_llvm(smallvec smallvec.c)

configure_file(sea.yaml sea.yaml @ONLY)
sea_discover_tests(smallvec)
