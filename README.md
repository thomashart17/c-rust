## Building

From the root of the project, run

```
mkdir build
cd build
cmake -DCMAKE_C_COMPILER=clang-14 -DCMAKE_CXX_COMPILER=clang++-14 ../
make
```

## Installing Rust with LLVM 14

To start, install rustc and rustup from the [rust web site](https://www.rust-lang.org/tools/install)

Next step is to set the rust version to one that used LLVM 14. It was first added in version 1.60 and last used in version 1.64.0. To do this for a version such as 1.64.0, use the following command:

```
rustup install <version>
rustup default <version>
```

To check that the update worked, run the following command to view the rustc version being used along with the version of LLVM being used:

```
rustc --version --verbose
```

The rust library contained in this project contains a `rust-toolchain.toml` to set the version to that needed for compatability with LLVM 14 and the required rust compiler flags. If building the project with additional crates, ensure to add a `rust-toolchain.toml` file in the crate directory setting the default toolchain channel to `nightly-2022-08-01`.

## Installing LLVM 14 Tools

First ensure that clang and lld are installed and based on LLVM version 14

```
sudo apt install clang-14 lld-14
```

To check installation, run

```
clang-14 --version
ld.lld-14 --version
```


## Build Process

### Generating C Headers From Rust Crate

Use CBindGen to generate C header files for Rust library. First install CBindGen

```
cargo install -force cbindgen
```

Next create a cbindgen.toml file in the crate's directory. For the test lib in this project,

```
cd /project-path/src/test-lib
echo -e "language = \"C\"\n\ninclude_guard = \"TEST_LIB_H_\"" > cbindgen.toml
```

Once configured, inside the same directory, run the cbindgen command

```
cbindgen --config cbindgen.toml --output ./inc/lib.h
```

This will generate a header file `lib.h` inside the crate's folder inside of a new `inc` folder.


### Building Rust in C

The Rust in CMake build toolchain was derived from the [CMakeRust repo](https://github.com/Devolutions/CMakeRust)

### LTO

Link Time Optimization was based off the resource available [here](https://blog.llvm.org/2019/09/closing-gap-cross-language-lto-between.html).

Before running LTO between rust and C, clang and lld must be installed as described in **Installing LLVM 14 Tools** 
Now to build the lto files

When building for LTO, certain flags are set for the rust compiler `rustc`, the LLVM C frontend and the LLVM linker.

#### Rustc flags

The flags needed for LTO with the rustc compiler are set in the `Cargo.toml` file and will be implemented by the cargo tool automatically. They are described here.

- `crate-type = static-lib` - Will generate a static system library as the output of crate compilation.
- `opt-level = 2` - Sets optimizer level 2
- `Clinker-plugin-lto` - Defers LTO optimization to the final linking stage. Passed as a profile `RUSTFLAG`.
- `Zemit-thin-lto=no` - Experimental flag to running thin LTO pass when using the LTO linker plugin. Passed as a profile `RUSTFLAG`.

#### C flags

The flags needed for LTO with the clang compiler are passed as compiler arguments. They are described here.

- `flto` - Defers LTO optimization to the final linking stage 
- `O2` - Sets optimizer level 2

#### Compiler flags

The flags needed for LTO with the lld linker are passed as arguments. They are described here

- `flto` - Enables LTO
- `fuse-ld=lld-14` - Enables using the LLVM linker LLD of version 14
- `Wl,--plugin-opt=-lto-embed-bitcode=post-merge-pre-opt` - Embeds the post merge, pre optimized bitcode to the resultant binary file
- `O2` - Set optimizer level 2

### Outputting LLVM

Once the binary is generated, it is necessary to extract the bitcode from the file. This is done using the `llvm-objcopy-14` and `llvm-dis-14` tool

```
objcopy a.out --dump-section .llvmbc=llvm.bc
llvm-dis-14 llvm.bc
```