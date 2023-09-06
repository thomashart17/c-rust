c_rust_llvm(smallvec-insert_many-fix smallvec-insert_many-fix.c)

configure_file(sea.yaml sea.yaml @ONLY)
sea_add_unsat_test(smallvec-insert_many-fix)
