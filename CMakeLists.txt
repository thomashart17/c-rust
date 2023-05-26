cmake_minimum_required(VERSION 3.19)

project(C-Rust)

# ---------- Local CMake Modules ----------- #
set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")


# ---------- Install Corrosion ----------- #
include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.3.5 # fixed tag, update manually
)

# Our preferred toolchian for which fat lto works
set(Rust_TOOLCHAIN nightly-2022-08-01)

# Set any global configuration variables such as `Rust_TOOLCHAIN` before this line!
FetchContent_MakeAvailable(Corrosion)


include(CTest)
enable_testing()

set(SEAHORN_ROOT "" CACHE PATH "Path to SeaHorn installation")
set(SEA_LINK "llvm-link-14" CACHE STRING "Path to llvm-link")
set(SEA_OPT "${SEAHORN_ROOT}/bin/seaopt" CACHE STRING "Path to seaopt binary")
set(SEA_PP "${SEAHORN_ROOT}/bin/seapp" CACHE STRING "Path to seapp binary")

set(LLVM_OBJCOPY "llvm-objcopy-14" CACHE STRING "Path to llvm-objcopy")
set(LLVM_DIS "llvm-dis-14" CACHE STRING "Path to llvm-dis")

set(LLVMIR_LINK ${SEA_LINK})
set(LLVMIR_OPT ${SEA_OPT})

set(CBINDGEN_TOML ${CMAKE_SOURCE_DIR}/cbindgen.toml)

configure_file(verify.py.in verify @ONLY)
set(VERIFY_CMD python3 ${CMAKE_CURRENT_BINARY_DIR}/verify)

include_directories(BEFORE ${CMAKE_CURRENT_BINARY_DIR}/include)
include_directories(${SEAHORN_ROOT}/include)

corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml)

add_subdirectory(src)

