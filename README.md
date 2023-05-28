# Builder and Stub

A Rust program that updates a resource in a Windows executable file using the Windows API functions. This project is based on the knowledge gained from [this blog post](https://genesisdatabase.wordpress.com/2011/01/11/builder-stub-how-to-create-your-own-builder-and-stub-in-c-using-eof/).

## Overview

The Builder and Stub project demonstrates how to programmatically update a specific resource within a Windows executable file. It provides a Rust implementation that leverages the Windows API functions to accomplish this task. The program is designed to work with resource types defined by the Windows operating system, such as RT_RCDATA.

The main functionality of the project includes:

- Copying a stub executable to a new file for resource updating.
- Prompting the user for input to fill a C-style struct with the provided details.
- Converting the struct into the appropriate format for resource updating.
- Updating the desired resource within the executable with the updated struct.

By following the steps outlined in the program, users can modify the resource data in the executable file, ensuring the updated information is available when the executable is run.

## Usage

1. Clone the repository:

```shell
git clone https://github.com/SourM1lk/builder_and_stub.git
```

2. Build the project:
```shell
cd builder_and_stub
cargo build --release
```
3. Run the Program:
```
cargo run --release
or
./builder.exe
./test.exe
```

## License

[![Beerware License](https://img.shields.io/badge/License-Beerware-yellow.svg)](https://en.wikipedia.org/wiki/Beerware)

This project is licensed under the Beerware License. If you found this project enjoyable or useful, and we meet someday, you can buy me a beer in appreciation. Cheers! 🍻
