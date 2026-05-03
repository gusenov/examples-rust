If you installed Rust through some other means, check whether Cargo is installed by entering the following in your terminal:

```
% cargo --version
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
```

# Creating a Project with Cargo

```
% cargo new hello_cargo
    Creating binary (application) `hello_cargo` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

…

It has also initialized a new Git repository along with a *.gitignore* file. 
Git files won’t be generated if you run `cargo new` within an existing Git repository; you can override this behavior by using `cargo new --vcs=git`.

Note: Git is a common version control system. 
You can change `cargo new` to use a different version control system or no version control system by using the `--vcs` flag. 
Run `cargo new --help` to see the available options.

…

Cargo expects your source files to live inside the [src](src) directory. 
The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. 
Using Cargo helps you organize your projects. 
There’s a place for everything, and everything is in its place.

…

One easy way to get that `Cargo.toml` file is to run cargo init, which will create it for you automatically.

# Building and Running a Cargo Project

```
% cargo build
   Compiling hello_cargo v0.1.0 (examples-rust/hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.38s
% ./target/debug/hello_cargo 
Hello, world!
```

…

[Cargo.lock](Cargo.lock). This file keeps track of the exact versions of dependencies in your project. 
This project doesn’t have dependencies, so the file is a bit sparse. 
You won’t ever need to change this file manually; Cargo manages its contents for you.

…

we can also use cargo run to compile the code and then run the resultant executable all in one command:

```
% cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello_cargo`
Hello, world!
```

…

Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:

```
% cargo check
    Checking hello_cargo v0.1.0 (examples-rust/hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
```

Why would you not want an executable? Often, `cargo check` is much faster than `cargo build` because it skips the step of producing an executable. If you’re continually checking your work while writing the code, using `cargo check` will speed up the process of letting you know if your project is still compiling! As such, many Rustaceans run `cargo check` periodically as they write their program to make sure it compiles. Then, they run `cargo build` when they’re ready to use the executable.

# Building for Release

When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations. This command will create an executable in *target/release* instead of *target/debug*. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run `cargo build --release` and benchmark with the executable in *target/release*.

[🔗](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html)
