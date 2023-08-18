c_rust_llvm(smallvec-insert_many-error smallvec-insert_many-error.c)

configure_file(sea.yaml sea.yaml @ONLY)
sea_add_sat_test(smallvec-insert_many-error)
