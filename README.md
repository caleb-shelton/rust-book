# Rust book
My notes and code for working through the [Rust book](https://doc.rust-lang.org/book/ch20-02-multithreaded.html) 🦀

# Chapter 1
* `fn main()` is where execution starts in all Rust programs
* `!` means using a macro
* Cargo is Rust's build system and package manager
* `cargo new` `cargo build` `cargo run` `cargo check` `cargo build --release`

# Chapter 2
* `use` for std and rand lib (an external crate)
* `let`
* `match`
* `loop`
* `read_line` `expect` `trim` `parse`
* `expect` returns value in brackets if `Result` is an `Err` value, if `Result` is an `Ok` value, it will return the value `Ok` is holding

# Chapter 3
## Variables and mutability
* `const` and type must be annotated!
* "Shadowing" or reusing immutable variables by defining `let` on the same variable name after it is defined, the second variable overshadows the first until it itself is overshadowed or the scope ends. Shadowing also lets use reuse the variable name but change the type and retain immutability after the transformation.

## Data Types
* Integers `i32` `u32`, etc
* Floating points `f64` `f32`
* Numeric operations, remainder `%`
* Boolean type `true` `false`
* Character type `char` defined using single quotes `let r = '🦀'`
* Compound types: tuples and arrays can group multiple values into one type
* Use pattern matching on a tuple to destructure it (get all the individual values into their own variables)
* Access the first element of a tuple, x, by using `x.0`
* The tuple without any values () has a special name, "unit". Expressions implicitly return the unit value if they don't return any other value
* Array elements must be of the same type
* Arrays in Rust have a fixed length
* Arrays are useful when you want your data allocated on the stack rather than the heap, or when you want to ensure you will always have a fixed number of elements
* A vector, is a similar type to an array provided by the standard library, but can grow and shrink
* You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so: `let a: [i32; 5] = [1, 2, 3, 4, 5];`
* Shorthand trick `let a = [3; 5];` defines `[3, 3, 3, 3, 3]`
* Access elements in an array using indexing `a[0]`
* If you use user input for specifying which element to pick and choose an out of bound ones Rust will panick with a run time error, rather than let you continue like in some other low-level languages that will let you access invalid memory. If you try to access out of bounds index without ambigious user input, Rust will catch this and fail during the compilation stage.

## Functions
* `main` function, entry point of many programs
* `fn` keyword
* snake case for function and variable names is the conventional style
* You must declare the type of every parameter in function definition
* Function bodies contain a series of statements optionally ending in an expression. Rust is an expression based language. A statement is for example `let y = 6;`. An expression is `3 * 2` or even just `6` in the statement `let y = 6;`. Expressions don't include ending semicolons (the semicolon in the previous example is a semicolon for the statement not for the `6` expression).
* Function returns `->` must specify type. You can return implicitly with the final expression in the block of the body of a function
* `return` keyword with a value

## Control flow
* `if` statement, needs a bool, will not try and convert non bool to bool, like JavaScript does, for example.
* `else` `else if`
* `match` is recommended over lots of `else if`s
* returning a value after a `break`
* `break` and `continue` apply by default to the innermost loop, however you can use loop labels so it can apply to the labelled loop. Loop labels are defined with a single quote and name, like so: `'counting_up: loop {}`
* `while`
* `for`
* Range `(1..4)`
* `rev()` to reverse the range

# Chapter 4
## Ownership
* Ownership is a set of rules that govern how a Rust program manages memory. If any of the rules are violated, the program won't compile.
* Main purpose of ownership is to manage heap data (as opposed to stack data)
* Ownership rules: Each value in Rust has an owner, there can only be one owner at a time, when the owner goes out of scope the value will be dropped
* 