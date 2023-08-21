c_rust_llvm(smallvec-drain-error smallvec-drain-error.c)

configure_file(sea.yaml sea.yaml @ONLY)
sea_add_sat_test(smallvec-drain-error)
