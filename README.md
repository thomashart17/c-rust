## Installing Rust with LLVM 14

To start, install rustc and rustup from the [rust web site](https://www.rust-lang.org/tools/install)

Next step is to set the rust version to one that used LLVM 14. It was first added in version 1.60 and last used in version 1.64.0. To do this for a version such as 1.64.0, use the following command:

```
rustup install 1.64.0
rustup default 1.64.0
```

To check that the update worked, run the following command to view the rustc version being used along with the version of LLVM being used:

```
rustc --version --verbose
```

## Generating C Headers From Rust Crate

Use CBindGen to generate C header files for Rust library. First install CBindGen

```
cargo install -force cbindgen
```

Next create a cbindgen.toml file in the crate's directory. For the test lib in this project,

```
cd /project-path/src/test-lib
touch cbindgen.toml
```

In this, add the following configuration for enabling C bindings and adding include gaurds

``` toml
language = "C"

include_guard = "TEST_LIB_H_"
```

Once configured, inside the same directory, run the cbindgen command

```
cbindgen --config cbindgen.toml --crate test-lib --output ./inc/lib.h
```

This will generate a header file `lib.h` inside the crate's folder inside of a new `inc` folder.


## Building Rust in C

Build toolchain derived from the [CMakeRust repo](https://github.com/Devolutions/CMakeRust)

To build:

```
mkdir build
cd build
cmake ..
make
```

To run from the build directory

```
./src/main/c-rust
```