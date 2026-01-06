# C Bindings for `AccumulativeHashSet`

This directory contains C bindings for the `AccumulativeHashSet` data structure from the `accumulative_hash` Rust crate. The bindings allow C programs to create, manipulate, and free instances of `AccumulativeHashSet<u64>`.

## Instructions

1. Install [CBindgen](https://github.com/mozilla/cbindgen?tab=readme-ov-file) if you haven't already:

   ```sh
   cargo install --force cbindgen
   ```

2. Build the C bindings and generate the header file by running:

   ```sh
   make build
   ```

3. The compiled dynamic library and the generated header file will be located in the `dylib` directory.

## Files

Depending on your operating system, the dynamic library file will have one of the following extensions:

- `.so` for Linux
- `.dylib` for macOS
- `.dll` for Windows

Files

- `libc_accumulative_hash_set.h`: The generated C header file containing the declarations for the C bindings.
- `libc_accumulative_hash_set.*`: The compiled dynamic library file.

## Usage

Include the generated header file in your C program and link against the compiled dynamic library to use the `AccumulativeHashSet` functionality.

```c
#include "libc_accumulative_hash_set.h"
```

The build it with:

```sh
gcc -o your_program your_program.c -Ipath/to/dylib -Lpath/to/dylib -lc_accumulative_hash_set
```