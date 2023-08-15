# Crate

Smallest amount of code that Rust compiler considers at a time

Crates can contain modules, and the modules may be defined in other files that 
get compiled with the crate

A crate can come in one of two forms: a binary crate or a library crate.

## binary crate
  - A program you can compile to an executable that you can run, such as a command-line program or a server
  - Each must have a function called main that defines what happens when the executable runs

## library crate
  - The crate does not have a `main` function and does not compile to an executable
  - The crate defines functionality intended to be shared with multiple projects


## crate root
  - A source file that the Rust compiler starts from and makes up the root module of your crate

## crate rules
 - package must have at least one crate
 - package could have either 0 or 1 library crate
 - package could have any number of binary crate

# Package

A bundle of one or more crates that provides a set of functionality

A package contains a `Cargo.toml` file that describes how to build those crates

Cargo is a package that contains the binary crate

The Cargo package also contains a library crate that the binary crate depends on

A package can contain as many binary crates as you like, but at most only one library crate
  - A package must contain at least one crate, whether that's a library or binary crate

Cargo convention
  - src/main.rs: the crate root of a binary crate with the same name as the package. 
  - src/lib.rs: the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. 

Cargo passes the crate root files to rustc to build the library or binary.

A package can have multiple binary crates by placing files in the src/bin directory
: each file will be a separate binary crate.
