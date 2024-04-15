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