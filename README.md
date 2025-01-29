# Bytepeep • [![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) ![Crates.io](https://img.shields.io/crates/v/bytepeep)

Bytecode in, optimized bytecode out.

### About

Bytepeep is a minimal bytecode peephole optimizer. Like any other peephole optimizer, it works by passing pairs of opcodes, peepholes, through a pattern matching algorithm to be checked against a set of rules and optimized if possible.

![diagram](diagram.png)

This is **not intended as a replacement for solidity's built-in optimizers**. Rather it should serve as a quick and easy tool for low-level smart contract development.

### Installation

**Install Rust & Cargo:**
`curl https://sh.rustup.rs -sSf | sh`

**Install bytepeep:**
`cargo install bytepeep`

**Run:**
`bytepeep <bytecode>`

### Todo

Contributions welcome!

- [ ] Handle multi-variable rules, (stack/memory dependent)
- [ ] Reassign jumps
- [ ] Handle different inputs
  - [ ] Mnemonics
  - [x] Huff
- [ ] Return tips along with optimized bytecode

### Disclaimer

This is experimental software and is provided on an "as is" and "as available" basis. We do not give any warranties and will not be liable for any loss incurred through any use of this codebase.
