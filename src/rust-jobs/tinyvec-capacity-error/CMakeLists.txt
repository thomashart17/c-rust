c_rust_llvm(tinyvec-capacity-error tinyvec-capacity-error.c)

configure_file(sea.yaml sea.yaml @ONLY)
sea_add_sat_test(tinyvec-capacity-error)
