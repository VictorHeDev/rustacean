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
