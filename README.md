# ckb-contract-bytes-without-a

This serves as an example showcasing the use of [Bytes](https://docs.rs/bytes/latest/bytes/) before CKB2023 upgrade.

Typically, Bytes crate requires atomic builtins, which is implemented via A extension in RISC-V. However, A extension is only available when CKB2023 upgrade arrives.

With [lib-dummy-atomics](https://github.com/xxuejie/lib-dummy-atomics) and some compiler flag changes, we can have Bytes without using any A extensions, making it possible to introduce Bytes before CKB2023 upgrades.

To build the project, make sure you have clang 16 and the latest stable Rust installed, the `riscv` target must also be there:

```
$ rustup target add riscv64imac-unknown-none-elf
```

Then you can build the contract as a normal Rust project:

```
$ cargo build --target=riscv64imac-unknown-none-elf --release
```

And run it via `ckb-debugger`:

```
$ ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/ckb-contract-bytes-without-a --script-version=1
Run result: -73
Total cycles consumed: 20077(19.6K)
Transfer cycles: 5209(5.1K), running cycles: 14868(14.5K)
```

Note that a run result of `-73`(or any other return value) is a success here. As a comparison, debugger would throw an error when the contract actually contains instruction from A extension:

```
ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/ckb-contract-bytes-without-a --script-version=1
Trace:
  /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.100/src/int/mul.rs:66:__muloti4
  ??:??:??
  ??:??:??
  ??:??:??
  ??:??:??
Error:
  InvalidInstruction { pc: 82690, instruction: 336213423 }
```
