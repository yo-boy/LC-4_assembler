# Description

This is an assembler for the LC-4 architecture written entirely in Rust, it was developed in tandem with the simulator for that same architecture, the specification for the architecture is not currently published anywhere, but it will be, and will be linked here when that is done.

# Usage

The assembler is a CLI tool that takes input assembly files in the specified format for LC-4 (examples can be found in this repo and the simulator repo), running the assembler with the `--help` flag gives usage information

```
Assembler for the LC-4 architecture.

Usage: assembler [input] [output]

Arguments:
  [input]   assembly input file [default: ./examples/hello.asm]
  [output]  binary output file [default: ./examples/out.bin]

Options:
  -h, --help     Print help
  -V, --version  Print version
```
