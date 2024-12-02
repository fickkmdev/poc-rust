# poc-rust

**Cargo** is an official Rust package management tool to create compile and execute Rust applications. It is similar to npm in Nodejs
* dependencies are configured in the toml file
* download the dependencies from crates.io
* Compile packages
* and upload dependencies into crates.io

### Command
**Create an application**
* Create a Running project
cargo new --bin *name*
* Create a library project
cargo new *name*

**Build and Run**
cargo build
cargo build --release

cargo run
cargo run argument1 argument2

**Test and Doc**
cargo test
cargo doc --open

**Add the dependencies**
cargo add *package*
Dependencies can be included in multiple ways
* You can specific name=version to get dependency from cates.io
* Specify name={git="url"} to get the dependencies github repository
* Include local path of an library using {path=“localpath”}

**Project Structure**
│   .gitignore
│   Cargo.lock
│   Cargo.toml
├───src
│       main.rs
└───target

**src folder**: Contains Rust code files, Contains main.rs and other modules files
**main.rs** is Root file, It entry execution file when you run the project using the cargo run command
**Cargo.toml** contains dependencies of a project add the below project metadata.
