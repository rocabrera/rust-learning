# **Guessing Game**

```rust
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

By default, Rust has a set of items defined in the _**standard library**_ that it brings into the scope of every program. This set is called the prelude, and you can see everything in it in the [standard library documentation](https://doc.rust-lang.org/std/prelude/index.html).

In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change.

The ```&``` indicates that this argument is a reference (**like variables, references are immutable by default**), which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.

The ```read_line``` function puts whatever the user enters into the string we pass to it, but it also returns a **Result** value. [Result](https://doc.rust-lang.org/std/result/enum.Result.html) is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.

Without the ```expect``` function to hangle the **Result** the program compiles but returns a warning. The right way to suppress the warning is to actually write error-handling code, but in our case we just want to crash this program when a problem occurs, so we can use ```expect```.

Cargo understands [Semantic Versioning](https://semver.org/) (sometimes called SemVer), which is a standard for writing version numbers. The specifier 0.8.5 is actually shorthand for ^0.8.5, which means any version that is at least 0.8.5 but below 0.9.0.

Another neat feature of Cargo is that running the cargo ```doc --open``` command will build documentation provided by all your dependencies locally and open it in your browser. 

The ```loop``` keyword creates an infinite loop. We’ll add a loop to give users more chances at guessing the number.