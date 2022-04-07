# Course Plan
Course plan.

## Week - 1:
1. Installation
2. Introduction to `cargo`
3. Introduction to Rust Syntax:
	- Variables: `let`, `mut`
	- Functions: `fn`
	- `String`, `str`, `format!`, `println!`
	- Arrays: `Vec`, `slice`
	- HashMap (or Dictionary): `HashMap`
	- Ownership, Borrowing
	- Control Flow: `if`, `if let`, `match`
	- Loops: `loop`, `while`, `while let`, `for`, iterators
#### Homeworks:
1. Write `fn hello(name:&str) -> String`
   - will return: "Hello -name-!"
2. Write `fn make_it_double(num:i32) -> i32`
   - will return: 4 when given 2
3. Write `fn multiply_pi(num:f32) -> f32`
   - will multiply the num with Pi(π) and return the result.
## Week - 2:
1. Introduction to: 
   - Enum
   - Result
   - Option
   - Struct
   - Traits
   - Macro
#### Homeworks:
1. Write `fn to_letter_grade(num:u8) -> String`
   - will return "AA" for 100, "BA" for 89 etc.
2. Write `fn log(level:LogLevel, msg:&str) -> String` 
   - LogLevel is an enum. Prints logs with level tag: `[WARN]: This is warning log.`
3. Write `Person {name:String, age: u8, gender: Gender}` 
   - Gender is an enum. Also implement `Display` trait for `Person` with format: `{name}({age})` ex: `Emin(24)`.
4. Write `display!(Person, "{}({})", name, gender)`
    - This macro will implement Display trait for Person with format you provide.
## Week - 3:
1. Crates, Tests
	- Crates: `use`, `mod`, `pub mod`, create your own crate
	- Usage of an example crate: [serde](https://docs.rs/serde/latest/serde/index.html)
	- Writing Tests
#### Homeworks:
1. Make `Person { name:String, age:u8, gender:Gender::Male }` struct transformable to JSON string with serde & serde_json crate:
	- Example: `{"name":"Emin","age":24,"gender":"Male"}`
2. Write 3 test cases for first task.
3. Write jsonified output of the `Person` struct to `person.json` file.
	- Writing to a file examples: https://doc.rust-lang.org/std/fs/struct.File.html
## Week - 4:
1. Closure
2. Iterator
	- `filter`
	- `map`
	- `nth`
	- `count`
	- `enumerate`
	- Fibonacci series with Iterator trait
3. Generics
4. Lifetimes
#### Homeworks:
1. Filter even numbers on a list
2. Multiply all numbers with 2 on a list
3. Implement an iterator which implements this algorithm: `f(n) = f(n-1) * f(n-2)` and `n > 0`
## Week - 5:
1. Thread, Mutex, Atomic, Channel
	- Fearless Concurrency
	- thread
	- thread w/ mutex
	- thread w/ atomic
	- thread w/ channel
#### Homeworks:
1. Calculate nth prime number with threads.
	- Find every given prime number on an array `&[u32]` with spawning a thread for each of it, and send the result with `std::sync::mpsc::Sender`.
## Week - 6:
1. Introduction to asynchronous programming:
	- `async-std`. `async`, `Future`
	- Example: async TCP server
#### Homeworks:
1. Write tcp chat server with `async-std`
	- Send each message to other connections.
	- Use Arc<Mutex<>> to share "user list" between the tasks.
	- Use 22222 port to make us test it easily.
