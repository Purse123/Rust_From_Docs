# Variable and Mutability
- let statement to create the variable
```rust
let apples = 5;
```
- variables are immutable by default, meaning once we give the variable a value, the **value won’t change**
- To make a variable mutable, we add mut before the variable name
```rust
let mut bananas = 5; // mutable
```
***
```rust
let mut guess = String::new();
```
- String::new, a function that returns a new **instance of a String**
- String::new is **growable** and **UTF-8 encoded** bit of text (the length of the string isn't known at compile time)
>> UTF-8 is commonly used text encoding for the web and modern computing system
>> UTF-8 uses **one to four bytes** to encode a character
>> UTF-16 uses **two bytes** (16 bit) to encode a character
- :: is a path separator, used to access associated function, constant, modules, etc...
- `new` is an associated function of the String type that creates a new, empty String.
- associated function is different from method
- associated function called on types themselves and method on specific instance of a type
