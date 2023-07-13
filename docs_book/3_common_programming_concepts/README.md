# **Common Programming Concepts**

## **Variables and Mutability**

By default, variables are immutable. Let’s explore how and why Rust encourages you to favor immutability and why sometimes you might want to opt out.

Although variables are immutable by default, you can make them mutable by adding ```mut``` in front of the variable name.

## **Constants**

Like immutable variables, constants are values that are bound to a name and are not allowed to change, but you are not allowed to use ```mut``` with constants. Constants aren’t just immutable by default—they’re always immutable.

You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

**Rust’s naming convention for constants is to use all uppercase with underscores between words.**


## **Shadowing**

You can declare a new variable with the same name as a previous variable. Rustaceans say that the first variable is shadowed by the second. 

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends. 

When shadowing, because we are using the ```let``` keyword again, we are effectively creating a new variable.

```rust
let spaces = "   ";
let spaces = spaces.len();
```

We get a compile-time error:
```rust 
let mut spaces = "   ";
spaces = spaces.len();
```

The error says we’re not allowed to mutate a variable’s type