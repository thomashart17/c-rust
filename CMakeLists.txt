cmake_minimum_required(VERSION 3.14)

project(C-Rust)

# ---------- Load Rust Cmake Tools ----------- #
set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")

enable_language(Rust)
include(CMakeCargo)
# -------------------------------------------- #

include(CTest)

enable_testing()

set(SEAHORN_ROOT "" CACHE PATH "Path to SeaHorn installation")
set(SEA_LINK "llvm-link-14" CACHE STRING "Path to llvm-link")
set(LLVMIR_LINK ${SEA_LINK})
set(SEA_OPT "${SEAHORN_ROOT}/bin/seaopt" CACHE STRING "Path to seaopt binary")
set(SEA_PP "${SEAHORN_ROOT}/bin/seapp" CACHE STRING "Path to seapp binary")
set(LLVMIR_OPT ${SEA_OPT})

set(CBINDGEN_TOML ${CMAKE_SOURCE_DIR}/cbindgen.toml)

set(AWS_C_COMMON_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/aws-c-common)

configure_file(verify.py.in verify @ONLY)
set(VERIFY_SCRIPT ${CMAKE_CURRENT_BINARY_DIR}/verify-c-common.sh)
set(VERIFY_CMD python3 ${CMAKE_CURRENT_BINARY_DIR}/verify)

include_directories(BEFORE ${CMAKE_CURRENT_BINARY_DIR}/include)

include_directories(${SEAHORN_ROOT}/include)

add_subdirectory(src)

