# **Cargo**
- cargo is rust **build system** and **package manager**.
- **TOML** (Tom’s Obvious, Minimal Language)

***
>> crates are hosted on crates.io, rust official package register
***
## As Build system:
it automates the process of compiling source code into an executable or library
- Compiling code
- managing build process    --> take care of compiling dependent crates(library)
- running tests and benchmarks
- custom build scripts

## As package manager
manage the external libraries(rust)
- dependency management
- version control     --> ensure that the correct version of libraries are used, based on dependency specification in "Cargo.toml" file
- publishing libraries  --> helps package and publish our libraries(crate) to crates.io
***
1. **Create project with Cargo**:
```sh
  $ cargo new hello_cargo
  $ cd hello_cargo
```
### file structure
  -> initilaize git
  -> src/main.rs
  -> Cargo.toml

***
>> You can change cargo new to use a different version control system or no version control system by using the --vcs flag. 
Run cargo new --help to see the available options.
***
2. **Cargo.toml**
  >> resource: https://doc.rust-lang.org/cargo/reference/manifest.html
- Cargo.toml file for each package is called its manifest.
- TOML format file.
- contains metadata that is needed to compile the package.

## **versioning**:
**MAJOR.MINOR.PATCH**
    MAJOR version when you make incompatible API changes
    MINOR version when you add functionality in a backward compatible manner
    PATCH version when you make backward compatible bug fixes
    
readme:
[package]
readme = "README.md"

- [package] configuring a package, As we add more information to this file, we’ll add here
- [dependencies] section to list dependencies. In Rust, packages referred as crates.
__________________________________________________________________________
3. **Building and Running a Cargo Project**
```sh
$ cargo build
```
- Running cargo build for the first time also causes Cargo to create a new file Cargo.lock. 
- **Cargo.lock** file keeps track of the exact versions of dependencies in your project.
- This command creates an **executable file in target/debug/[proj_name]** rather than in your current directory
- After building we **execute** by **./target/debug/[proj_name]**

```sh
$ cargo run
```
- Compile the code, then run the executable (build + run).

```sh
$ cargo check
```
- checks code to make sure it compiles but doesn’t produce an executable.
- without producing a binary to check for errors.

```sh
$ cargo build --release 
```
- to compile it with optimizations
- target/release instead of target/debug.
***
>> - in simple projects, Cargo doesn’t provide a lot of value over rustc
>> - for multiple files or a dependency, it’s easier for Cargo coordinate the build.
