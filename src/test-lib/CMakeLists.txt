add_custom_command(
    OUTPUT test-lib-bindings
    COMMAND cbindgen ARGS --config cbindgen.toml --output ./inc/lib.h
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Generating test-lib c bindings"
)

add_custom_target(test-lib-bindings_target ALL DEPENDS test-lib-bindings)

cargo_build(NAME test-lib)
