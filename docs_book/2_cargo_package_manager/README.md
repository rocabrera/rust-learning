# **Cargo package Manager**

Cargo is Rust’s build system and package manager.

To create a project usind cargo:
```sh
cargo new <project_name>
```

Cargo will generate two files and one directory for us: a _Cargo.toml_ file and a src directory with a _main.rs_ file inside. It will also initialize a new Git repository along with a ._gitignore_ file. Git files won’t be generated if you run cargo new within an existing Git repository.

Cargo's TOML file:
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

The last line, [dependencies], is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as **crates**.


## **Building and Running a Cargo Project**

To **build** using cargo:
```sh
cargo build
```
This command creates an executable file in _target/debug/<projec_name>_. Running cargo build for the first time also causes Cargo to create a new file at the top level: Cargo.lock. This file keeps track of the exact versions of dependencies in your project. This project doesn’t have dependencies, so the file is a bit sparse. You won’t ever need to change this file manually; Cargo manages its contents for you.

To **build** & **run** the program using cargo:
```sh
cargo run 
```

To check for errors (do not compile):
```sh
cargo check 
```