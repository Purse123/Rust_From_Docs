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
***
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
***
# Ensuring Reproducible Builds with the Cargo.lock File
- it means no matter when or where project is built, the **resulting binary or output will be same**, given the same source code and build environment.
- when building project, Cargo figures out all versions of dependencies fiting criteria then writes to Cargo.lock file.
- When build project in future, Cargo will see Cargo.lock file exists, will use the versions specified.
***
# Updating a Crate to Get a New Version
- to update a crate, Cargo provides "update" command , which ignores Cargo.lock file and figure out latest versions specificated in Cargo.toml
- Cargo then write versions to Cargo.lock file
```sh
$ cargo update
    Updating crates.io index
    Updating rand v0.8.5 -> v0.8.6
```
- Cargo ignores the 0.9.0 release
- To use any version in the 0.9.x series, you’d have to update the Cargo.toml file
```toml
[dependencies]
rand = "0.9.0"
```
- next time you run cargo build, Cargo will update the registry of crates
***
# Generating a Random Number
- Rng is trait, it defines methods to generate random number generator implement.
```rust
use rand::Rng;
```
- trait must be in scope to use method.
```rust
let secret_number = rand::thread_rng().gen_range(1..=100);
```
***
## Suppose i am importing like 
```rust
use rand::{Rng, thread_rng};
```
## I should write
```rust
let secret_number = thread_rng().gen_range(1..=100);
```
## so it is just like namespace in C++
***

1. **rand::thread_rng**
*rng = Random Number Generator*
- this function provides rng **local** to the **current thread**.
- this means each **thread gets its own rng**
- rng of thread_rng is seeded by OS, randomness based on system time, hardware event, etc
- Alternativly, we can use
```rust
let mut rng = rand::thread_rng();
let random_num = rng.gen_range(1..=100);
```

2. **gen_range**
- this method is provided by **Rng** trait
########################################################
*As i understand*
>> major concept of crate, trait, function and method
>> - rand is crate
>> - thread_rng is function which implement trait
>> - rng is trait
>> - gen_range is method

>> - trait is just like interface in java. for eg, assume two object **Dog** and **Cat**. I want to implement "speak method" it is different to each other but is defined on both.
```rust
interface Speakable {
  void speak();
}
class Dog implements Speakable {
    public void speak() {
        System.out.println("Woof!");
    }
}
class Cat implements Speakable {
    public void speak() {
        System.out.println("Meow!");
    }
}
```
>> - Rust doesn’t have **classes**, but **structs** and **enums** can implement traits.
>> #########################################
>> - gen_range method is defined as part of the Rng trait, which is found in rand crate.
## more deeper
1. rand crate is a comprehensive library for random number generation, It provides various tools, traits, and implementations for generating random values
2. Traits in the rand Crate:
- rng trait is defined in rand crate. it specify common interface for random number generation
- other trait eg: RngCore, SeedableRng, etc
3. rng trait
- it define varity of method like
    i. **gen_range**: generate random numbers within specified range. 
    ii. **gen**: generate random values of different type
    iii. **fill**: fill buffer with random bytes
4. Random Number Generator Implementations:
- **rand crate** includes various concrete random number generator types that implement the Rng trait. common ones are:
    i. ThreadRng: rng which is local to current thread and seeded by OS.
    ii. StdRng: it is deterministic and seeded with specific value.
    iii. SmallRng: smaller but faster rng, useful for certain application.

# Hierarchy
1. rand crate
│
├── Rng trait
│   ├── gen_range
│   ├── gen
│   ├── fill
│   └── ...
│
└── Random Number Generators
    ├── ThreadRng (implements Rng)
    ├── StdRng (implements Rng)
    ├── SmallRng (implements Rng)
    └── ...
2. for std
std (crate)
│
├── prelude (module)
│   ├── Clone (trait)
│   ├── Copy (trait)
│   ├── ...
│
├── cmp (module)
│   ├── Ordering (enum)
│   ├── PartialOrd (trait)
│   ├── Ord (trait)
│   └── ...
│
├── collections (module)
│   ├── Vec (type)
│   ├── HashMap (type)
│   ├── ...
│
├── io (module)
│   ├── Read (trait)
│   ├── Write (trait)
│   └── ...
└── ...

# Comparing guess and secret number
1. import the module into the scope
```rust
use std::cmp::Ordering;
```
- **Ordering is enum** with variants Less, Greater and Equal
2. cmp method compare the two value, it takes the reference of the value which we want to compare with.
```rust
match guess.cmp(&secret_number) {
  Ordering::Less => println!("Too small!"),
  Ordering::Greater => println!("Too big!"),
  Ordering::Equal => println!("You win!"),
}
```
- here comparing guess and secret_number

## There is error due to data type conflict
```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```
- program already have a variable named guess but also we create a variable of name guess. bcz, rust allows us to shadow the previous value of guess with a new one.
- **shadowing** allow to reuse the variable rather than forcing to create unique variable.

```rust
guess.trim().parse()
```
- here guess refers to **original variable** i.e. **string**.
- **trim** method on string eliminate whitespace at the beginning and end.
>> when value is 5 it store as 5\n. On Windows, its 5\r\n. **trim** eliminate the \n or \r\n. (\r is carriage return)
- **parse** method converts string to another type.
>> In this case it is guess: u32 i.e. 32-bit unsigned integer.
- parse only works on characters that can logically be converted to numbers. that is why we use expect() to handle potential failure
>> secret_number should also be u32 for comparision.
