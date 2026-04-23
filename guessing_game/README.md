# Increasing Functionality with a Crate

Cargo understands [Semantic Versioning](http://semver.org/) (sometimes called SemVer), which is a standard for writing version numbers. The specifier 0.8.5 is actually shorthand for ^0.8.5, which means any version that is at least 0.8.5 but below 0.9.0.

Cargo considers these versions to have public APIs compatible with version 0.8.5, and this specification ensures that you’ll get the latest patch release that will still compile with the code in this chapter. Any version 0.9.0 or greater is not guaranteed to have the same API as what the following examples use.

…

When we include an external dependency, Cargo fetches the latest versions of everything that dependency needs from the registry, which is a copy of data from [Crates.io](Crates.io). Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.

# Ensuring Reproducible Builds

Cargo has a mechanism that ensures that you can rebuild the same artifact every time you or anyone else builds your code: Cargo will use only the versions of the dependencies you specified until you indicate otherwise. For example, say that next week version 0.8.6 of the rand crate comes out, and that version contains an important bug fix, but it also contains a regression that will break your code. To handle this, Rust creates the Cargo.lock file the first time you run cargo build, so we now have this in the guessing_game directory.

When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the Cargo.lock file exists and will use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at 0.8.5 until you explicitly upgrade, thanks to the Cargo.lock file. Because the Cargo.lock file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.

# Updating a Crate to Get a New Version

When you do want to update a crate, Cargo provides the command `update`, which will ignore the *Cargo.lock* file and figure out all the latest versions that fit your specifications in *Cargo.toml*. Cargo will then write those versions to the *Cargo.lock* file. Otherwise, by default, Cargo will only look for versions greater than 0.8.5 and less than 0.9.0. If the `rand` crate has released the two new versions 0.8.6 and 0.999.0, you would see the following if you ran `cargo update`:

```
$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.85.0 compatible version
    Updating rand v0.8.5 -> v0.8.6 (available: v0.999.0)
```

Cargo ignores the 0.999.0 release. At this point, you would also notice a change in your *Cargo.lock* file noting that the version of the `rand` crate you are now using is 0.8.6. To use `rand` version 0.999.0 or any version in the 0.999.x series, you’d have to update the *Cargo.toml* file to look like this instead (don’t actually make this change because the following examples assume you’re using `rand` 0.8):

```
[dependencies]
rand = "0.999.0"
```

The next time you run `cargo build`, Cargo will update the registry of crates available and reevaluate your `rand` requirements according to the new version you have specified.

…

🔗

[Programming a Guessing Game](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html)