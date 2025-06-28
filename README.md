# WasmOS: An Experimental OS for Running WebAssembly

WasmOS is an experimental x86_64 operating system designed from the ground up to run userspace applications compiled to WebAssembly. Built upon the excellent foundational kernel from Philipp Oppermann's ["Writing an OS in Rust"](https://os.phil-opp.com/) series, this project extends it with a complete toolchain and runtime for executing apps written in C, C++, Rust and AssemblyScript.

![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue) ![Languages](https://img.shields.io/badge/languages-Rust%20%7C%20C++%20%7C%20WASM-orange)

## Core Features

*   **WebAssembly Runtime:** Integrates the `tinywasm` engine directly into the kernel to load and execute `.wasm` binaries.
*   **Userspace Applications:** Supports a full separation between kernel space and a sandboxed "userspace" running inside the WASM VM.
*   **Multi-Language Support:** Includes a custom toolchain for compiling applications written in C, C++, Rust, and AssemblyScript to WASM using the OS's ABI.
*   **Custom Standard Libraries:** A from-scratch implementation of key C (`<stdlib.h>`, `<stdio.h>`, etc.) and C++ standard library features (`<vector>`, `<iostream>`, `<cstdlib>`, etc.) that acts as a compatibility layer between application code and the kernel's ABI. Basic standard library features are implemented for Rust and AssemblyScript as well.
*   **Simple VFS:** Implements a basic virtual filesystem with device files for I/O (`/dev/stdin`, `/dev/stdout`, `/dev/serial0`).
*   **Automated Build System:** A custom Python script (`foreman.py`) orchestrates the compilation of all userspace applications and packages them into the final OS image.

## Architecture

WasmOS takes a "unikernel-style" approach. Applications are compiled against a specific kernel ABI and bundled directly into the OS image. This avoids the need for a complex dynamic linker and allows for a simple, robust system.

The architecture is composed of several layers:

1.  **The Kernel (Rust):** Based on `blog_os`, this layer provides core functionality:
    *   Memory management (paging and heap allocation).
    *   Interrupt handling (keyboard, timer).
    *   A simple VFS and device driver model (`inode.rs`).

2.  **The Syscall ABI (Rust):** The kernel exposes a set of low-level functions to the WASM runtime. These act as the system call interface. This is defined in `src/wasm.rs` using `imports.define`. Apps never directly make syscalls (although they can!), they simply use our implementations of the standard library for the specific language, which makes these on the inside. Key "syscalls" include:
    *   `putchar`, `getchar` for console I/O.
    *   `malloc`, `free`, `ptrsize` for memory management inside the WASM instance.
    *   `timesinceboot`, `cputime` for timekeeping.
    *   `runapp` to launch other WASM applications.
    *    many more!

3.  **The WASM Runtime (`tinywasm`):** The sandboxed execution engine that parses and runs the `.wasm` binaries, providing the bridge to the kernel's syscall ABI.

4.  **The Custom Standard Library (`stdlib/`):** This is the key compatibility layer that allows standard C/C++ code to run on WasmOS. It reimplements standard headers to map high-level functions to the kernel's low-level ABI. For example, a call to `std::cout << "Hello"` in C++ is translated by this library into a series of `putchar` calls to the kernel.

5.  **The Build System (`foreman.py` & Makefiles):** The `Foreman` script automates the toolchain, compiling each C/C++/AssemblyScript application with the correct flags (`-nostdlib`, `--target=wasm32`) and custom headers. It then uses a Rust build script (`build.rs`) to embed the compiled `.wasm` binaries directly into the kernel executable.

## Application Showcase

The OS comes with several demo applications that showcase its capabilities. The default application is a simple command-line interface made in C++, which you can use to launch the other demo applications (which are included in the OS during the default build process) by typing their name and pressing enter. These applications include:
| Application Name    | Language         | Description                                                                                             |
| ------------------- | ---------------- | ------------------------------------------------------------------------------------------------------- |
| `cli-cpp`           | C++              | The default shell. A simple command-line interface that can launch other applications using `std::system`. |
| `collatz`           | C++              | A performance-intensive Collatz Conjecture solver that makes heavy use of the custom `std::vector`.     |
| `vec-test-cpp`      | C++              | A comprehensive test suite for the custom `std::vector` implementation, verifying its correctness.        |
| `sysinfo`           | C++              | A utility to display system time information by calling the kernel's timekeeping ABI (`std::time`, `std::clock`). |
| `helloworld-cpp`    | C++              | A classic "Hello, World!" demonstrating basic I/O with the custom `std::iostream` library.              |
| `fib`               | C                | A program to calculate Fibonacci numbers, showing support for standard C with `stdio.h` and `stdlib.h`.  |
| `alloc-demo`        | C                | Demonstrates the kernel's memory management ABI by using `malloc`, `realloc`, and `free`.               |
| `lld-test`          | C                | A test case specifically for `printf`'s `long long int` (`%lld`) format specifier.                      |
| `helloworld-c`      | C                | The C equivalent of "Hello, World!", using the custom `stdio.h` library.                                |
| `helloworld-rs`     | Rust             | A "Hello, World!" program compiled from Rust to WASM, using a minimal custom Rust `std` library.        |
| `helloworld-as`     | AssemblyScript   | A "Hello, World!" program showing toolchain support for AssemblyScript.                                 |

## Building and Running

### Prerequisites

*   A nightly Rust toolchain (`rustup default nightly`). Note: Currently, you must stay on Rust 1.89 nightly, and can't upgrade to 1.90+ due to a bug in the `x86_64` crate, which is actively being worked on.
*   The `bootimage` cargo tool (`cargo install bootimage`).
*   `QEMU` for emulation (`sudo apt install qemu-system`).
*   `clang` and `clang++` for compiling the C/C++ applications.
*   `npm` and `assemblyscript` for the AssemblyScript application.

You can run the `install.sh` script on Debian-based systems to install most dependencies.

### Build Steps

1.  **Build the Userspace Applications:**
    The `foreman.py` script compiles all applications in the `apps/` directory and places the resulting `.wasm` files into the `rootfs/Applications` directory.

    ```bash
    python3 foreman.py
    ```
    *Note: This step requires `clang`, `clang++`, and `npm` to be installed.*

2.  **Build and Run the Kernel:**
    Once the applications are built, you can build and run the entire OS in QEMU with a single command:

    ```bash
    cargo run
    ```

    This will compile the kernel, use `build.rs` to embed the applications from the `rootfs`, and launch the resulting image in QEMU.

## Original Project and License

This project is a fork and extension of **Philipp Oppermann's** excellent [Writing an OS in Rust](https://os.phil-opp.com/) series. The original kernel code and concepts are a result of that work. This project would not be possible without it.

Both my work and the original blog_os are dual-licensed under the MIT license and the Apache License, Version 2.0. You can choose either at your option.

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details. Any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.