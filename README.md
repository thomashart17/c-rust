## Installing Rust with LLVM 14

To start, install rustc and rustup from the [rust web site](https://www.rust-lang.org/tools/install)

Next step is to set the rust version to one that used LLVM 14. It was first added in version 1.60 and last used in version 1.64.0. To do this for a version such as 1.64.0, use the following command:

`
rustup install 1.64.0
rustup default 1.64.0
`

To check that the update worked, run the following command to view the rustc version being used along with the version of LLVM being used:

`
rustc --version --verbose
`


