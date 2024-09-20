# **Guessing game explain**
1. 
```rust
use std::io;
```
- To obtain user input and then print the output we need io library in scope
- The io library comes from standard library ->std
- By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude
2.
```rust
fn main() {
}
```
- fn to define function
3.
```rust
println!("");
```
- **println!** is a macro that prints a string to the screen
4.
```rust
let mut guess = String::new();
```
- **let** statement to create the variable
- variables are immutable by default
```rust
let x = 5;  // immutable by default
x = 6;      // throw a compiler error, `x` is immutable

let mut x = 5;  // mutable
x = 6;          // This is valid -> mutable
```
- String::new, a function that returns a new **instance of a String**
- String::new is **growable** and **UTF-8 encoded** bit of text (the length of the string isn't known at compile time)
- :: is a path separator, used to access associated function, constant, modules, etc...
- `new` is an associated function of the String type that creates a new, empty String.
- associated function is different from method
- associated function called on types themselves and method on specific instance of a type
5.
```rust
io::stdin()
    .read_line(&mut guess)
    .expect("failled to read line");
```
- std::io::stdin
- read_line() method on standard input handle to get input from user.
- &mut guess as argument, to store user input, append to the variable
- .expect() Handling Potential Failure with Result
6.
```rust
println!("You guessed: {}", guess);
```
- {} work as placeholder
- When printing the result of evaluating an expression
- println!("x = {x} and y + 2 = {}", y + 2);
_____________________________________________________
> generate random number
1. include the **rand** crate as a dependency in **Cargo.toml**
>> - A crate is a collection of Rust source code files. The project we’ve been building is a binary crate, which is executable
>> - rand crate is a library crate, which contains code that is intended to be used in other programs and can’t be executed on its own
-  Semantic Versioning / SemVer
```rust
[dependencies]
rand = "0.8.5"
```
>> When include external dependency, Cargo fetches the latest versions of everything, which is a copy of data from Crates.io
```rust
```
