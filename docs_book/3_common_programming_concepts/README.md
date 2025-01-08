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
Result:
```txt
The value of x in the inner scope is: 12
The value of x is: 6
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


# **Data Types**

Rust is a statically typed language, which means that it must know the types of all variables at compile time. There are two data type subsets: 

- scalar: Represents a single value. Rust has four primary scalar types: **integers**, **floating-point** numbers, **Booleans**, and **characters**. 
- compound: Can group multiple values into one type. Rust has two primitive compound types: **tuples** and **arrays**.

### **Integers**

<div align="center">

| Length   | Signed | Unsigned |
|----------|--------|----------|
| 8-bit    | `i8`   | `u8`     |
| 16-bit   | `i16`  | `u16`    |
| 32-bit   | `i32`  | `u32`    |
| 64-bit   | `i64`  | `u64`    |
| 128-bit  | `i128` | `u128`   |
| arch     | `isize`| `usize`  |

</div>

The ***isize*** and ***usize*** types depend on the architecture of the computer your program is running on, which is denoted in the table as "arch": 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture. The primary situation in which you’d use ***isize*** or ***usize*** is when indexing some sort of collection.

### **Floating Point**

Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

### **Boolean**

As in most other programming languages, a Boolean type in Rust has two possible values: true and false. Booleans are one byte in size. The Boolean type in Rust is specified using bool.

### **Character**

Specified with single quotes, as opposed to string literals, which use double quotes. Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust.

```rust 
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

### **Tuple**

A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

```rust 
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access.

```rust 
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

### **Arrays**

Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length. Arrays are useful when you want your data allocated on the stack, rather than the heap or when you want to ensure you always have a fixed number of elements.

```rust 
fn main() {
    let _months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
}
```

It is also possible to initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets.

```rust 
fn main() {
    let a = [3; 5];
}
```

The array named a will contain 5 elements that will all be set to the value 3 initially. This is the same as writing ```let a = [3, 3, 3, 3, 3];``` but in a more concise way.

To access data in the array it is used the square brackets symbols:
```rust 
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

# **Functions**