#### Rust Course
### General Commands to work with Rust and Cargo
- Initialize your application:
    `cargo init`
    * With this command will create a src folder, .gitignore with target folder and Cargo.toml file (Like a package.json in JS)

- Run your application:
    `cargo run`
    * With this command will create target folder, will compile the src/main.rs file and put the executable file into debug folder and run it;
    * This command will run the code was compiled once, if some modification was made and it wasn't built, with _cargo run_ will compile the last compiled version.

- Check your application:
    `cargo check`
    * With this command, the rust compiler will verify what was modified in relation to codebase

- Build non-production version:
    `cargo build`
    * With this command will compile the code inside of src/main.rs and create an unoptimize executable file

- Build a release version:
    `cargo release`
    * With this command will create a release folder inside of target folder, compile src/main.rs and create the executable file optimized inside release folder

### General concepts:
*  In Rust, variables are immutable by default.
- `let foo = 5;` // immutable variable
- `let mut bar = 5;` // mutable variable