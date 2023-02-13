## Installing Rust with LLVM 14

To start, install rustc and rustup from the [rust web site](https://www.rust-lang.org/tools/install)

Next step is to set the rust version to one that used LLVM 14. It was first added in version 1.60 and last used in version 1.64.0. To do this for a version such as 1.64.0, use the following command:

```
rustup install 1.60.0
rustup default 1.60.0
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

To run the compiled file

```
cd /project/dir/
cd build 
./src/main/c-rust
```

To find size of file

```
du -h src/main/
```

Result: `5.0 M`

## LTO

Link Time Optimization was based off the resource available [here](https://blog.llvm.org/2019/09/closing-gap-cross-language-lto-between.html).

First ensure that clang and lld are installed and based on LLVM version 14

```
sudo apt install clang-14 lld-14
```

To check installation, run

```
clang-14 --version
ld.lld-14 --version
```

Now to build the lto files

```
cd /project/dir/
mkdir build
cd build 
rustc --crate-type=staticlib -O -C linker-plugin-lto -o libtest.a ../src/test-lib/src/lib.rs 
clang-14 -flto -c -O2 ../src/main/main.c
clang-14 -flto -fuse-ld=lld-14 -O2 main.o -L . -ltest
```

To check size:

```
du -h a.out
```

Result: `4.7 M`

To extract the bitcode out, run the last command as 

```
clang-14 -flto -fuse-ld=lld-14 -Wl,--plugin-opt=-lto-embed-bitcode=optimized -O2 main.o -L . -ltest
```

And extract the bitcode by:

```
objcopy a.out --dump-section .llvmbc=llvm.bc
llvm-dis-14 llvm.bc
```

**NOTE** The above currently does not work with anything built by rustc. To test the process, use the `test.c` file:

```
clang-14 -flto -O2 -fuse-ld=lld-14 -Wl,--plugin-opt=-lto-embed-bitcode=optimized ../src/main/test.c
objcopy a.out --dump-section .llvmbc=llvm.bc
llvm-dis-14 llvm.bc
head -5 llvm.ll
```

To try to fix the above problem, you can attempt to use the `-Zemit-thin-lto=no` flag. It is expiremental so a nightly build of rust must be used. The one used in the following demonstration is `nightly-2022-08-01`. Download using the steps described above in *Installing Rust with LLVM 14*.

Now, follow the steps above but building the rust object file with the added flag.


```
cd /project/dir/
mkdir build
cd build 
rustc --crate-type=staticlib -O -C linker-plugin-lto -Zemit-thin-lto=no -o libtest.a ../src/test-lib/src/lib.rs 
clang-14 -flto -c -O2 ../src/main/main.c
clang-14 -flto -fuse-ld=lld-14 -Wl,--plugin-opt=-lto-embed-bitcode=optimized -O2 main.o -L . -ltest
objcopy a.out --dump-section .llvmbc=llvm.bc
llvm-dis-14 llvm.bc
```


# # Emitting LLVM IR

For C code, run below and find it at `build/main.s`

```
cd /project/dir/
mkdir build
cd build 
clang-14 -flto -c -O2 -emit-llvm -S ../src/main/main.c
```

For rust, run below and find it located in `src/test-lib/target/deps/test_lib*.ll`

```
cd /project/dir/
cd src/test-lib
cargo rustc -- --emit=llvm-ir
```