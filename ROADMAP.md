# General todo list, not prioritized
- [x] Add an '@' op which will evaluate an address on the stack
- [x] Add a way to clear the stack
- [x] Add a way to get the size of the stack
- [x] Add rotn, a way to swap the top of the stack with the nth item
- [x] Add pluck, a way to pull the nth item from the stack to the top
- [x] Add a method to see if the interpreter should exit
- [ ] Fix issue with nested if statements
- [ ] Fix REPL mode launching from interpreter. Calling `repl` command messes stuff up, also having an error in the repl doesn't increment the PC and just explodes the app. Maybe repl should reset pc?
- [ ] Remove noop from interpreter
- [ ] Deprecate auto-format
- [ ] Add a way to do comments that go until the end of the line
- [ ] Add a way to get documentation for a single word
- [ ] Make it so instead of reading address off the stack, you instead can execute them. Add in ways to get the type of a stack variable, e.g. `is-number?`, `is-address?`, `is-string?`, etc. This will allow you to use definitions in the compile mode.  Perhaps that's unnecessary. 
- [ ] Add way to put program counter on stack, then add an instruction that jumps the program counter to a given address. Add a way to get the size of the stack.
- [ ] Make setting 'exit' exit the program evaluate loop.
- [ ] Add loading of other files
