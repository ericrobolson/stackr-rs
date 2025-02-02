//! A stack-based interpreter written in Rust. Meant to be embedded in your application.
//! [Visit the repository for more information](https://github.com/ericrobolson/stackr-rs).
//!
//!
//! Basic usage:
//! ```rust
//! use stackr_rs::*;
//!
//! let code = "1 1 +";
//! let mut interpreter = Interpreter::new(());
//! interpreter.evaluate(code, None).unwrap();
//! println!("1 1 + ={}", interpreter.pop_number().unwrap());
//! ```
//!
//! [See more built-in words](https://github.com/ericrobolson/stackr-rs?tab=readme-ov-file#useful-words).
//!
//! Example with custom built-in words:
//! ```rust
//! use stackr_rs::*;
//!
//! let state: u32 = 0;
//! let mut interpreter = Interpreter::new(state);
//!
//! interpreter.register_builtin(
//!     "increment-state",
//!     "--",
//!     "Increments the state.",
//!     "increment-state",
//!     |interpreter| {
//!         interpreter.state += 1;
//!         Ok(())
//!     },
//! );
//!
//! interpreter.register_builtin(
//!     "get-state",
//!     "-- n",
//!     "Gets the state.",
//!     "get-state",
//!     |interpreter| {
//!         interpreter.push_number(interpreter.state as f32);
//!         Ok(())
//!     },
//! );
//!
//! let code = r#"
//! print-stack
//! increment-state
//! get-state
//! "The state has been modified!"
//! print-stack
//! "#;
//!
//! println!("State before execution: {:?}", interpreter.state);
//! interpreter.evaluate(code, None).unwrap();
//! println!("State after execution: {:?}", interpreter.state);
//! ```
//!
//! Example of a custom word:
//! ```
//! : squared
//! "n -- n^2"
//! "Squares the top of the stack"
//! "2 squared"
//! dup *
//! ;
//!
//! 2 squared
//! print-stack
//! ```
//!
//! See the examples directory for more examples or the [README](https://github.com/ericrobolson/stackr-rs/blob/main/README.md) for more information.

mod interpreter;

pub use interpreter::*;
