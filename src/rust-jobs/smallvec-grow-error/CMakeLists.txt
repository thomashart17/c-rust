c_rust_llvm(smallvec-grow-error smallvec-grow-error.c)

configure_file(sea.yaml sea.yaml @ONLY)
sea_add_sat_test(smallvec-grow-error)
