cmake_minimum_required(VERSION 3.0)

project(C-Rust)

# ---------- Load Rust Cmake Tools ----------- #
set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")

enable_language(Rust)
include(CMakeCargo)
# -------------------------------------------- #

include(CTest)

enable_testing()

set(SEAHORN_ROOT "/usr" CACHE PATH "Path to SeaHorn installation")
set(SEA_LINK "llvm-link" CACHE STRING "Path to llvm-link")
set(LLVMIR_LINK ${SEA_LINK})
set(SEA_OPT "${SEAHORN_ROOT}/bin/seaopt" CACHE STRING "Path to seaopt binary")
set(SEA_PP "${SEAHORN_ROOT}/bin/seapp" CACHE STRING "Path to seapp binary")
set(LLVMIR_OPT ${SEA_OPT})







add_subdirectory(src)

