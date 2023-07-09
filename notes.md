# Chapter 1
Rust style is to indent with 4 spaces, not a tab.

`!()` calls a macro instead of a normal function.

`rustc` is fine for compiling small, simple programs. As the project grows, you'll want to use Cargo. Cargo is Rust's build system and package manager. Cargo handles a lot of tasks such as building your code, downloading the libraries that your code depends on, and building those libraries (dependencies). 
Add dependencies under the `[dependencies]` in the Cargo.toml file. 

## Cargo
Create a new project by using `cargo new <project name>`
Build a Cargo project by running `cargo build`
Two ways to run a cargo project is by building the project and then calling the file `./target/debug/hello_cago` or by using `cargo run`.
Cargo also has `cargo check` for quickly checking your code to make sure that it can compile without actually producing the executable.
When your project is finally ready for release, use `cargo build --release` to compile it. This will create the executable in `target/release` instead of `target/debug`. 

# Chapter 3
## Variables and Mutability
Variables by default are immutable. Can use the `mut` keyword.  

## Constants
Constants must be declared with the `const` keyword and with their type annotated. 

## Shadowing 
Declaring a new variable with the same name as a previous variable is called `shadowing`. You can shadow a variable by using the same variable's name and repeating the use of `let`. Shadowing is different from using `mut` because the variable declared with `let` is still immutable. Using `let` creates a new variable and uses additional space. 
```rust
fn main() {

    let x = 5;
    let x = x + 1;  // x == 6

    {
        let x = x * 2;  // x == 12
    }
    // x == 6
}
```

## Data Types
2 data type subsets: scalar and compound. 

### Scalar types
Represents a single value. 4 primary scalar types: integers, floating-point numbers, Booleans, and characters. 

#### Integer types
Signed integer types start with `i` instead of `u` for unsigned integers. Rust defaults integers to `i32`. A scenario where you would use `isize`/`usize` is when indexing a collection. 
| Length  | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

#### Floating-Point types
Rust's floating point types are `f32` and `f64` (default). All floating point types are signed.

#### Boolean type
1 byte in size, specified by using `bool` for `true` and `false` values. 

#### Character type
Primitive alphabetic type. `char` literals are specified with single quotes, not double quotes. They are 4 bytes in size and can represent more than just ASCII (like emojis). 

### Compound types
Can group multiple values into one type. Rust has 2 primitive compound types: tuples and arrays.

#### Tuple type
Tuples have a fixed length and can be created by writing a comma-separated list of values inside parentheses. Tuple elements can directly by accessed by using a period (.) followed by the index of the value.
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1); 
    let (x, y, z) = tup;    // use destructuring 
    let six_point_four = tup.1;     // access index 1
    let one = tup.2;
}
```

#### Array type
Every element of an array must have the same type. Arrays have a fixed length and are written as a comma-separated list inside square brackets. Arrays are useful when you want your data allocated on the stack rather than the heap or when you want to ensure you always have a fixed number of elements. An array isn't as flexible as a vector, which is a similar collection type that can grow and shrink in size. Vectors are more commonly used than arrays. 
```rust
let a: [i32; 5A] = [1, 2, 3, 4, 5];
let a = [3; 5]; // initial value is 3, fill the array up to length 5 [3, 3, 3, 3, 3]
```

## Functions
`main` is the entrypoint for many programs, and the `fn` keyword is used to declare new functions. 

### Statements and Expressions
Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resultant value. Expressions do not include ending semicolons. 

### Functions with Return Values
We must declare their type after an arrow (->). Most functions return the last expression implicitly, but you can early return explicitly.

## Control Flow
You can use multiple conditions by combining `if`, `else`, and `else if`. Having too many `else if`s can clutter your codebase, so a branching construct called `match` is often used. `if` is an expression so we can use it on the right side of a `let` statement to assign the outcome to a variable. 

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };     // 5
}
```

### Loops
Loop labels can be used to distinguish between loops. Loops with a conditional can be managed with a `while`. Looping through a collection is commonly done with a `for` loop. 

```rust
fn main() {
    for number in (1..4).rev() {    // Range sequence
        println!("{number}!");
    }

    println!("Liftoff!!!");
}
```

