#!/usr/bin/env python3
"""
Script to autogenerate the files necessary to add a new
job to the project.
"""
import os
import sys

BASE = os.path.dirname(os.path.realpath(__file__))

def generate_cargo(directory, job):
    with open(f"{directory}/Cargo.toml", "w") as file:
        file.write("cargo-features = [\"profile-rustflags\"]\n\n")
        file.write("[package]\n")
        file.write(f"name = \"{job}-lib\"\n")
        file.write("version = \"0.1.0\"\n")
        file.write("edition = \"2021\"\n\n")
        file.write("[lib]\n")
        file.write("crate-type = [\"staticlib\"]\n")
        file.write("path = \"lib.rs\"\n\n")
        file.write("[dependencies]\n")
        file.write("libc = \"0.2\"\n")
        file.write("sea_rs_common = { path = \"../../sea_rs_common\" }\n\n")
        file.write("[profile.dev]\n")
        file.write("linker = \"lld\"\n\n")
        file.write("panic = \"abort\"\n")
        file.write("opt-level = 2\n\n")
        file.write("rustflags = [\n")
        file.write("    \"-Clinker-plugin-lto\",\n"),
        file.write("    \"-Clinker=clang-14\",\n")
        file.write("    \"-Clink-arg=-fuse-ld=lld\",\n")
        file.write("    \"-Zemit-thin-lto=no\",\n")
        file.write("]\n\n")
        file.write("[profile.release]\n")
        file.write("panic = \"abort\"\n")
        file.write("opt-level = 2\n\n")
        file.write("rustflags = [\n")
        file.write("    \"-Clinker-plugin-lto\",\n"),
        file.write("    \"-Clinker=clang-14\",\n")
        file.write("    \"-Clink-arg=-fuse-ld=lld\",\n")
        file.write("    \"-Zemit-thin-lto=no\",\n")
        file.write("]\n")


def generate_toolchain(directory):
    with open(f"{directory}/rust-toolchain.toml", "w") as file:
        file.write("[toolchain]\n")
        file.write("channel = \"nightly-2022-08-01\"\n")


def generate_cmake_lists(directory, job):
    with open(f"{directory}/CMakeLists.txt", "w") as file:
        file.write(f"c_rust_llvm({job} {job}.c)\n\n")
        file.write(f"sea_add_unsat_test({job})\n")


def generate_rust(directory, job):
    with open(f"{directory}/lib.rs", "w") as file:
        file.write("#[no_mangle]\n")
        file.write(f"pub extern \"C\" fn {job}() -> T {{\n")
        file.write("    T\n")
        file.write("}\n")


def generate_c(directory, job):
    with open(f"{directory}/{job}.c", "w") as file:
        file.write("#include <stdio.h>\n\n")
        file.write("#include \"seahorn/seahorn.h\"\n")
        file.write("#include \"inc/lib.h\"\n\n")
        file.write("int main() {\n\n")
        file.write("    return 42;\n")
        file.write("}\n")


def update_cmake(job):
    cmake_file = os.path.join(BASE, "src", "CMakeLists.txt")
    with open(cmake_file, "a") as file:
        file.write(f"add_subdirectory(rust-jobs/{job})\n")


def main():
    if len(sys.argv) < 2:
        print("Error: Please specify a name for your job.")
    else:
        args = len(sys.argv)
        for i in range(1, args):
            job = sys.argv[i]
            directory = os.path.join(BASE, "src", "rust-jobs", job)
            if os.path.isdir(directory):
                print(f"Error: The job \"{job}\" already exists.")
                continue
            os.mkdir(directory)
            generate_cargo(directory, job)
            generate_toolchain(directory)
            generate_cmake_lists(directory, job)
            generate_rust(directory, job)
            generate_c(directory, job)
            update_cmake(job)
            print(f"Created job: {job}")


if __name__ == "__main__":
    main()
