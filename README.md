# Arch Template

This is a arch template that can provide a very tiny kernel runtime.

## How to use

Please ensure that you have installed the cargo-generate. You can using the following command to install.

```shell
cargo install cargo-generate
```

## Generate the tiny kernel runtime.

```shell
cargo generate --git https://github.com/Byte-OS/arch-template.git
```

then you can write your commands to the main function in main.rs.

## How to run

```shell
# RUN riscv64
make ARCH=riscv64 run
# RUN aarch64
make ARCH=aarch64 run
# RUN x86_64
make ARCH=x86_64 run
# RUN loongarch64, You should ensure that you have loongarch64 qemu higher than version 8.2.0
make ARCH=loongarch64 run
```
