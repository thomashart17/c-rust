# Create a target to generate Rust to C bindings using cbindgen
function(generate_c_bindings TARGET RUSTLIB)
endfunction()

# Generate target to compile C files, link with Rust library, and produce LLVM IR
# The Rust library is expected to be called ${TARGET}-lib
function(c_rust_llvm TARGET SRC_FILES)
    set(RUST_LIB_TARGET ${TARGET}-lib)

    # XXX all Rust libraries are loaded together from the workspace
    corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml 
                           CRATES ${RUST_LIB_TARGET})


    # build std library; require panic to abort without printing anything
    # when using no_std you can specify in crate's cmake to use custom panic 
    if ("CUSTOM_PANIC_NO_STD" IN_LIST ARGN)
      corrosion_set_cargo_flags(${RUST_LIB_TARGET}
          "-Zbuild-std=panic_abort,std"
          "-Zbuild-std-features=default"
      )
    else()
      corrosion_set_cargo_flags(${RUST_LIB_TARGET}
          "-Zbuild-std=panic_abort,std"
          "-Zbuild-std-features=panic_immediate_abort"
      )
    endif()
    # corrosion_set_cargo_flags(${RUST_LIB_TARGET}
    #     "-Zbuild-std=panic_abort,std"
    #     "-Zbuild-std-features=panic_immediate_abort"
    # )



    # generate Rust to C bindings
    add_custom_command(
		    OUTPUT  ${CMAKE_CURRENT_BINARY_DIR}/inc/lib.h
		    COMMAND cbindgen ARGS --config ${CBINDGEN_TOML} --output ${CMAKE_CURRENT_BINARY_DIR}/inc/lib.h
		    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
		    DEPENDS ${RUST_LIB_TARGET}
		    COMMENT "Generating C bindings for rust code for target ${TARGET}"
		)

    add_custom_target(${TARGET}-cbindgen DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/inc/lib.h)

    add_executable(${TARGET} ${SRC_FILES})
    add_dependencies(${TARGET} ${TARGET}-cbindgen)
    # include path for header files generated by cbindgen
    target_include_directories(${TARGET} PUBLIC ${CMAKE_CURRENT_BINARY_DIR}) 

    set(ARCH x86_64-unknown-linux-gnu)
    # add vacuity check def for jobs that directly
    # include seahorn header
    target_compile_definitions(${TARGET} PRIVATE VACCHECK)

    # compile C files with LTO 
    # do not perform optimizations, but do not add optnone marker in LLVM IR
    # target is set to linux, this might not be required
    target_compile_options(${TARGET} PUBLIC
        -flto
        -O1
        -Xclang -disable-llvm-optzns
        --target=${ARCH}
    )

    # link rust into C
    # Rust library is expected to link all other libraries it uses
    target_link_libraries(${TARGET} ${RUST_LIB_TARGET})
    # link seahorn intrinsics
    target_link_libraries(${TARGET} sea-lib.LIB)


    # LTO during linking
    # Embed bitcode prior to optimization
    # Still do optimization for the executable (XXX is this needed?)
    target_link_options(${TARGET} PUBLIC
        -flto
        -fuse-ld=lld
        -Wl,--plugin-opt=-lto-embed-bitcode=post-merge-pre-opt
        --target=${ARCH}
        -O1
    )

    SET(BC_FILE ${TARGET}_llvm.bc)
    SET(LL_FILE ${TARGET}_llvm.ll)

    add_custom_command(
        OUTPUT  ${BC_FILE}
        COMMAND ${LLVM_OBJCOPY} ARGS ${CMAKE_CURRENT_BINARY_DIR}/${TARGET} --dump-section .llvmbc=${BC_FILE}
        DEPENDS ${TARGET}
        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
        COMMENT "Copying llvm bitcode sections from executable for ${TARGET}"
    )

    add_custom_command(
        OUTPUT  ${LL_FILE}
        COMMAND ${LLVM_DIS} ARGS ${CMAKE_CURRENT_BINARY_DIR}/${BC_FILE}
        DEPENDS ${BC_FILE}
        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
        COMMENT "Outputting disassembled program post merge pre opt llvm code for ${TARGET}"
    )

    add_custom_target(${TARGET}.ir ALL DEPENDS ${LL_FILE})
    
    set_target_properties(${TARGET}.ir PROPERTIES
        LLVMIR_DIR ${CMAKE_CURRENT_BINARY_DIR}
        LLVMIR_FILES ${BC_FILE}
        LLVMIR_LL_FILES ${LL_FILE}
    )
endfunction()

function(sea_get_file_name VAR LLVMIR_TARGET)
  get_property(DIR TARGET ${LLVMIR_TARGET} PROPERTY LLVMIR_DIR)
  get_property(FILES TARGET ${LLVMIR_TARGET} PROPERTY LLVMIR_FILES)
  set(${VAR} "${DIR}/${FILES}" PARENT_SCOPE)
endfunction()

# Run unit proof, expecting unsat
function(sea_add_unsat_test TARGET)
  sea_get_file_name(BC ${TARGET}.ir)
  add_test(NAME "${TARGET}_unsat_test" COMMAND ${VERIFY_CMD} ${VERIFY_FLAGS} --expect=unsat ${BC})
endfunction()

# Run unti proof, expecting sat
function(sea_add_sat_test TARGET)
  sea_get_file_name(BC ${TARGET}.ir)
  add_test(NAME "${TARGET}_sat_test" COMMAND ${VERIFY_CMD} ${VERIFY_FLAGS} --expect=sat ${BC})
endfunction()

function(sea_disable_unsat_test TARGET)
  set_tests_properties("${TARGET}_unsat_test" PROPERTIES DISABLED TRUE)
endfunction()

function(sea_disable_sat_test TARGET)
  set_tests_properties("${TARGET}_sat_test" PROPERTIES DISABLED TRUE)
endfunction()

function(sea_discover_tests TARGET)
  sea_get_file_name(BC ${TARGET}.ir)
  cmake_path(SET bcpath "${BC}")
  cmake_path(GET bcpath PARENT_PATH bc_dir_path)
  cmake_path(APPEND bc_dir_path "CTestTestfile.cmake" OUTPUT_VARIABLE ctest_file)
  string(CONCAT target_ir ${TARGET} ".ir")
  add_custom_command(
    OUTPUT ${ctest_file}
    COMMAND ${CMAKE_COMMAND}
    ARGS
    -D "TARGET=${TARGET}"  # name of target
    -D "TEST_TARGET=${BC}"  # full path of linked bitcode IR file
    -D "VERIFY_CMD=${VERIFY_CMD}"  # verify cmd to use
    -D "SYS_LLVM_NM=${LLVM_NM}"
    -D "SYS_AWK=${AWK}"
    -D "SYS_CPP_FILT=${CPP_FILT}"
    -P ${EXTRACT_TEST_CMD}  # extract tests script
    DEPENDS ${BC}
    COMMENT "Writing discovered tests to ${ctest_file}"
    VERBATIM
    )
  # Add discovered tests to directory TEST_INCLUDE_FILES
  set_property(DIRECTORY
    APPEND PROPERTY TEST_INCLUDE_FILES "${ctest_file}"
  )
  # Test discovery should run only after the linking of the final IR. For this,
  # we need to create a custom target (discovery) that depends on the custom cmd
  # (discovery) output and then declare that the custom target (discovery)
  # depends on the custom cmd (linking).
  add_custom_target("${TARGET}_disc_tests" ALL  # ALL adds it to default target list in Ninja
    DEPENDS ${ctest_file})
  add_dependencies("${TARGET}_disc_tests" ${target_ir})
endfunction()
