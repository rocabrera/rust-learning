# **Anatomy of a Rust Program**

```rust
fn main() {
    println!("Hello, world!");
}
```

These lines define a function named **main**. The main function is special: it is always the first code that runs in every executable Rust program.

The ```println!``` command calls a **Rust macro**. If it had called a function instead, it would be entered as println (without the !). For now, you just need to know that using a ! means that you’re calling a macro instead of a normal function and that macros don’t always follow the same rules as functions.

## **Compiling and Running Are Separate Steps**

To compile:
```zsh
rustc <file_name>.rs
```
To execute:
```zsh
./<file_name>
```

Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.