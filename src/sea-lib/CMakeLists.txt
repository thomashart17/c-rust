add_library(sea-lib.LIB STATIC seahorn.c)
target_compile_definitions(sea-lib.LIB PRIVATE VACCHECK)
target_compile_options(sea-lib.LIB PUBLIC
	-flto
        -O1
        -Xclang -disable-llvm-optzns
        --target=x86_64-unknown-linux-gnu
)
