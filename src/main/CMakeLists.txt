function(c_rust_llvm TARGET SRC_FILES)
    add_executable(${TARGET} ${SRC_FILES})

    target_compile_options(${TARGET} PUBLIC 
        "-flto"
        "-O2"
    )

    target_link_libraries(${TARGET} test-lib)

    target_link_options(${TARGET} PUBLIC
        "-flto"
        "-fuse-ld=lld-14"
        "-Wl,--plugin-opt=-lto-embed-bitcode=post-merge-pre-opt"
        "--target=x86_64-unknown-linux-gnu"
        "-O2"
    )

    SET(BC_FILE ${TARGET}_llvm.bc)

    add_custom_command(
        OUTPUT ${TARGET}-bc
        COMMAND llvm-objcopy-14 ARGS ${CMAKE_CURRENT_BINARY_DIR}/${TARGET} --dump-section .llvmbc=${BC_FILE}
        DEPENDS ${TARGET}
        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
        COMMENT "Copying llvm bitcode sections from executable for ${TARGET}"
    )

    add_custom_command(
        OUTPUT ${TARGET}-llvm
        COMMAND llvm-dis-14 ARGS ${CMAKE_CURRENT_BINARY_DIR}/${BC_FILE}
        DEPENDS ${TARGET}-bc
        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
        COMMENT "Outputting disassembled program post merge pre opt llvm code for ${TARGET}"
    )

    add_custom_target(${TARGET}_LLVM_OUT ALL DEPENDS ${TARGET}-llvm)
endfunction()

set(ADD_SRC add.c seahorn.c)
c_rust_llvm(add "${ADD_SRC}")

set(VEC_SRC vec.c seahorn.c)
c_rust_llvm(vec "${VEC_SRC}")

set(POINTER_SRC pointer.c seahorn.c)
c_rust_llvm(pointer "${POINTER_SRC}")

set(ENUM_AS_PARAM_SRC enum_as_param.c seahorn.c)
c_rust_llvm(enum_as_param "${ENUM_AS_PARAM_SRC}")

set(OPTION_SRC option.c seahorn.c)
c_rust_llvm(option "${OPTION_SRC}")
