c_rust_llvm(tinyvec-arrayvec tinyvec-arrayvec.c)

configure_file(sea.yaml sea.yaml @ONLY)
sea_discover_tests(tinyvec-arrayvec)
