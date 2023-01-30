
add_executable(c-rust main.c)

# if(WIN32)
#     target_link_libraries(helloworld test-lib ws2_32 userenv)
# elseif(CMAKE_SYSTEM_NAME MATCHES "Darwin")
#     target_link_libraries(helloworld test-lib resolv)
# else()
#     target_link_libraries(helloworld test-lib pthread dl m)
# endif()

target_link_libraries(c-rust test-lib)
