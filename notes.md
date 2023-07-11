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
# Chapter 4 - Understanding Ownership
Ownership is a set of rules that govern how a Rust program manages memory. For Rust, memory is managed through a system of ownership and a set of rules that the compiler checks. 

## The Stack and the Heap
Parts of ownership will be described in relation to stack vs. heap. The stack stores values in the order that it gets them, and removes the items that are "stacked on the top" like a stack of plates (LIFO). The heap is less organized because when you put data on the heap, you request a certain amount of space. The internal memory allocator will find an empty spot on the heap that is big enough for the space that you requested, and it returns a pointer (address of location). This is called "allocating on the heap" or sometimes called "allocating." If we want the data stored at the memory address, all we need is a pointer to where the data is stored (the pointer). Using the stack is faster, accessing data in the heap is slower. However, optimizations can be made to minimize the amount of duplicate data on the heap, cleaning up unused data on the heap so you don't run out of space. These are problems that ownership can solve. 

### Ownership Rules
* Each value in Rust has an owner
* There can only be one owner at a time
* When the owner goes out of scope, the value will be dropped

#### The String type
String literals are convenient, but they aren't suitable for every situation in which we may want to use text. One reason is that they're immutable. Another is that not every string value can be known when we write our code. Rust has a second string type called `String` which can take user input and store it, for example. This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. You can also create a `String` from a string literal by using the `from` function:
```rust
let mut s = String::from("Hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{}", s);  // this will print "hello, world!"
```
So this begs the question, why can `String` be mutated but String literals cannot? 

#### Memory and Allocation
We know the contents of a string literal, which is why they are fast and efficient. With the `String` type, in order to support mutable, growable text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. 
1. Memory must be requested from the memory allocator at runtime
2. We need a way of returning this memory to the allocator when we're done with our `String`
In other words, we need exactly one `allocate` with exactly one `free` to properly manage the memory on the heap. In Rust, memory is automatically returned once the variable that owns it goes out of scope. When a variable goes out of scope, Rust calls a special function called `drop` automatically at the closing curly brace (end of scope). This is similar in C++, where the pattern of deallocating resources at the end of an item's lifetime is sometimes called RAII.

```rust
let s1 = String::from("hello");
let s2 = s1;
```
Instead of copying and creating a new string for `s2`, both `s1` and `s2` here share the same pointer to "hello" on the heap. In other words, we do not copy the data on the heap that the pointer refers to. What Rust actually does is that it invalidates `s1`. Instead of being called a shallow copy, it is called a `move`. This prevents the double deallocation of memory, known as a "double free" error. 

#### Variables and Data interacting with clone
If we do want to deeply copy heap data of a `String`, we can use a method called `clone`.

#### Stack-only data: Copy
Integers have a known size at compile time, so are stored entirely on the stack. As a general rule, any group of simple scalar values can implement `copy`, and nothing that requires allocation or is some form of resource can implement `copy`. 

#### Ownership and Functions
Passing in a variable to a function will move or copy.

#### Return values and scope
Returning values can also transfer ownership. The ownership of a variable follows this pattern: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable. What do we do when we want to let a function use a value but not take ownership? We can use a Rust feature for using a value without transferring ownership, called `references`. 

## References and Borrowing
A `reference` is like a pointer in that it's an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference. `&` represent `references`, and they allow you to refer to some value without taking ownership of it. The opposite of referencing by using `&` is `dereferencing`, which is accomplished with the dereference operator, `*`. 

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);    // &s1 refers to the value of s1 but does not own it

    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &String) -> usize {  // s is a reference to a String
    s.len();
}   // s goes out of scope, but because it doesn't have ownership of what it refers to, it is not dropped
```

We call the action of creating a reference `borrowing`. We can think of it like if a person owns something, you can borrow it from them. When you're done, you have to give it back because you don't own it. 

### Mutable References
Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. This feature/restriction prevents data races at compile time. We cannot have a mutable reference while we have an immutable one to the same value. 

```rust
fn main() {
    let mut  s = String::from("hello");
    change(&mut s);
}
fn change(some_string: mut &String) -> { 
    some_string.push_str(", world");
}   
```

### Dangling References
A `dangling pointer` is a pointer that references a location in memory that may have been given to someone else--by freeing some memory while preserving a pointer to that memory. Rust will prevent this by throwing a compile-time error. 

## Rules of References
* At any given time, you can have either one mutable reference or any number of immutable references
* References must always be valid

## The Slice Type
Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice does not have ownership. 

### String Slices
A reference to part of a `String`. Rather than referencing the entire `String`, we only reference a portion of the `String`. 
```rust
let s = String::from("hello world");
let hello = &s[0..5];
let slice = &s[..5];    // also has the value of "hello"

let world = &s[6..11];
let world_slice = &s[6..];  // this is also the same as "world"
```

```rust
fn first_word(s: &String) -> &str {
    // given a sentence, return the first word. If there is only one word in the sentence then return the word.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

### String literals as slices
```rust
fn first_word(s: &String) -> &str{} // can be further improved
fn first_word(s: &str) -> &str{} // can be further improved
```

Defining a function to take a string slice instead of a reference to a `String` makes our API more general and useful.


# Chapter 5 - Using Structs to Structure Related Data
A `struct` is a custom data type that lets you package together and name multiple related values that make up a meaningful group.

## Defining and Instantiating Structs
Structs are similar to Tuples, and both can hold multiple related values of different types. Structs are more flexible than Tuples, and you don't need to rely on the order of the data. To construct a Struct:

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com"); // mutate value
}
```

## Using the Field Init Shorthand
```rust
fn build_user(email: String, username: String) -> User {
    // Create new instance of the User struct
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```
It is sometimes useful to create a new instance of a struct that includes most of the values from another instance. You can do this by using the "struct update" syntax.

```rust
fn main(email: String, username: String) -> User {
    // -- snip --
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // specifies that any remaining fields come from corresponding fields in user1
    }
}
```

## Using Tuple Structs without named fields to create different types
Tuple structs are structs that look similar to tuples. They have the added meaning the struct name provides, but don't have names associated with their fields. Instead, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name, and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant. 

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

## Ownership of Struct Data
It's possible for structs to store references to data owned by something else, but to do so we have to use `lifetimes`. `Lifetimes` ensure that the data referenced by a struct is valid for as long as the struct is. 

## Method syntax
Methods are similar to functions except they are defined within the context of a struct (or an enum or a trait object). Their first parameter is always `self`, which represents the instance of the struct the method is being called on. Rust does not implement automatic `getters` for struct fields. 

### Where is the `->` operator like in C or C++?
Rust has automatic referencing and dereferencing and does not use a -> operator. Rust will automatically add in `&`, `&mut`, or `*` when an object's method is called. '

### Associated Functions
Associated functions that don't have `self' as their first parameter don't need an instance of the type to work with. Associated functions that aren't methods are often used for constructors that will return a new instance of the struct.

### Multiple `impl` blocks
Each struct is allowed to have mutiple `impl` blocks as a way to organize code. 






