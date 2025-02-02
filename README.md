# stacker-rs

A stack-based interpreter written in Rust.
Heavily inspired by Forth.
Useful for embedding a scripting layer in your application.

To add to your project, run:
```
cargo add stackr-rs
```

To use the main branch, add this to your `Cargo.toml`:
```
[dependencies]
# Reference git repo
stacker-rs = { git = "https://github.com/ericrobolson/stacker-rs.git" } 
```

## Examples
Run any of the examples with `cargo run --example <example-name>` or with `make example-<example-name>`.

### Example of adding two numbers
```
1 1 +
```

### Example of a custom defined word 
```
: squared
    "n -- n^2"
    "Squares the top of the stack"
    "2 squared"
    dup *
;

2 squared
print-stack
```

### Example of an if statement
```
"Example of if"
print-stack
drop

1 if 
    "evaluated when true!"
    print-stack
    drop
end

"Now we do an example of an else"
print-stack
drop

0 if 
    "Not ran"
    print-stack
    drop
else
    "else is ran"
    print-stack
    drop
end
```

### Example of a loop
```
0
begin
    print-stack

    dup 10 <=
    if
        drop
        "loop finished"
        print-stack
        break
    end
    1 + 
loop
```

Note: a loop will never exit unless `break` is called.

### Example of custom built-in words
```rust
let mut state: u32 = 0;
    let mut interpreter = Interpreter::new(state);

    interpreter.register_builtin(
        "increment-state",
        "--",
        "Increments the state.",
        "increment-state",
        |interpreter| {
            interpreter.state += 1;
            Ok(())
        },
    );

    interpreter.register_builtin(
        "get-state",
        "-- n",
        "Gets the state.",
        "get-state",
        |interpreter| {
            interpreter.push_number(interpreter.state as f32);
            Ok(())
        },
    );

    let code = r#"
    print-stack
    increment-state
    print-stack
    get-state
    print-stack
    "#;

    interpreter.evaluate(code, None).unwrap();
    println!("State after execution: {:?}", interpreter.state);
```

### Example of a REPL
```rust
// Rust code
interpreter.start_repl();
```

## Useful words

- `.` Noop operation. Used for denoting line breaks in the program.
- `print-stack` - Prints the stack
- `documentation` - Prints all registered words and their documentation
- `drop`, `dup`, `swap`, `over` - Various stack manipulation words
- `begin`, `loop`, `break` - Loop control words
- `if`, `else`, `end` - If statement control words
- `:`, `;` - Compilation words
- `repl`, `repl-end` - REPL control words

There are more words available, run the `documentation` word to see all of them or run `cargo run --example print_documentation` to see all of them.


# Features 
- Custom built-in words
- Custom words
- Loops
- If statements
- Stack manipulation words
- REPL mode

# Non-features (right now)
- Comments
- For loops
- While loops
- Switch statements
