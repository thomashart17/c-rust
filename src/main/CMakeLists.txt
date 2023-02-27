
add_executable(c-rust main.c seahorn.c)

# if(WIN32)
#     target_link_libraries(helloworld test-lib ws2_32 userenv)
# elseif(CMAKE_SYSTEM_NAME MATCHES "Darwin")
#     target_link_libraries(helloworld test-lib resolv)
# else()
#     target_link_libraries(helloworld test-lib pthread dl m)
# endif()

target_compile_options(c-rust PUBLIC 
    "-flto"
    "-O2"
)

target_link_libraries(c-rust test-lib)

target_link_options(c-rust PUBLIC
    "-flto"
    "-fuse-ld=lld-14"
    "-Wl,--plugin-opt=-lto-embed-bitcode=post-merge-pre-opt"
    "-O2"
)

SET(BC_FILE llvm.bc)

add_custom_command(
    OUTPUT c-rust-bc
    COMMAND llvm-objcopy-14 ARGS ${CMAKE_CURRENT_BINARY_DIR}/c-rust --dump-section .llvmbc=${BC_FILE}
    DEPENDS c-rust
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    COMMENT "Copying llvm bitcode sections from executable"
)

add_custom_command(
    OUTPUT c-rust-llvm
    COMMAND llvm-dis-14 ARGS ${CMAKE_CURRENT_BINARY_DIR}/${BC_FILE}
    DEPENDS c-rust-bc
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    COMMENT "Outputting disassembled program post merge pre opt llvm code"
)

add_custom_target(LLVM_OUT ALL DEPENDS c-rust-llvm)