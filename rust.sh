#!/bin/sh

#Install Rust ecosystem
#curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

# From https://doc.rust-lang.org/stable/book/ch01-01-installation.html
# You will also need a linker, which is a program that Rust uses to join its compiled outputs into one file. It is likely you already have one. If you get linker errors, you should install a C compiler, which will typically include a linker. A C compiler is also useful because some common Rust packages depend on C code and will need a C compiler.
#
# On macOS, you can get a C compiler by running:
#
# $ xcode-select --install
#
# Linux users should generally install GCC or Clang, according to their distribution’s documentation. For example, if you use Ubuntu, you can install the build-essential package.

# Installing rustup on Windows
#
# On Windows, go to https://www.rust-lang.org/tools/install and follow the instructions for installing Rust. At some point in the installation, you’ll be prompted to install Visual Studio. This provides a linker and the native libraries needed to compile programs. If you need more help with this step, see https://rust-lang.github.io/rustup/installation/windows-msvc.html

rustc --version

rustup update

#to compile & run local executable
#cargo run

#to install the executable from this repo on path
#cargo install --path .

# uninstall the executable
#cargo uninstall

#rustup uninstall # uninstall the rust ecosystem

