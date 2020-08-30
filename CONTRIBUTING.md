# Contributing
Contributions to FossXO are welcome! This page describes how to get started
developing for this game.

This guide describes the tools and procedures used to ensure work is done
efficiently while maintaining consistent code quality. Don't worry if this
guide seems overwhelming; we use automated tools to check pull requests
so you do not have to worry if a step is accidentally skipped.


## Quick Reference
Below are some common commands used while developing ths game.

```bash
# Build the game, user manual, and run the quick tests.
cargo make dev

# Run the complete test suite.
cargo make test

# Check the project before making a pull request.
cargo make pr-check

# Run the game (optionally in release mode)
cargo run
cargo run --release

# Open the API documentation.
cargo doc --open
```

Useful web links:
* [Game Design Document](https://fossxo.github.io/gdd/)
* [Amethyst Documentation](https://book-src.amethyst.rs/master/)


## Getting Started

### 1. Install Rust
This library is developed using the [Rust programming language](https://www.rust-lang.org/).
Install Rust on your platform per the instructions on the Rust website.

If you are new to Rust, [The Rust Programming Language](https://doc.rust-lang.org/stable/book/)
book is a great place to start learning about the language.

### 2. Install Platform Specific Tools

#### Windows
[Inno Setup](https://jrsoftware.org/isdl.php) is used to package the game on Windows.
Install the latest stable release of Inno Setup. The extra tools such as
QuickStart Pack are not needed.

Finally, add the location where you installed Inno Setup to your path.
When finished you should be able to run it from the command line,
for example:

```bash
> ISCC /?
Inno Setup 6 Command-Line Compiler
```

#### Ubuntu / Debian
Install the following packages:

```bash
# apt install gcc pkg-config openssl libasound2-dev cmake \
    build-essential python3 libfreetype6-dev libexpat1-dev \
    libxcb-composite0-dev libssl-dev libx11-dev
```


### 3. Install Common Tools
Install the `cargo-make` utility using `cargo`:

```bash
cargo install cargo-make
```

### 4. Build the Game
Finally, you can build the game:
```
# Build the game, user manual, and run the quick tests.
cargo make dev
```

It can take a while for the first build to complete. Additional Rust
packages and tools are downloaded and built during this porcess.
Once the build is finished and the tests pass you are ready to
start modifying code.


## What to Work On
Feel free to take a look at the issue tracker for tasks and bugs to tackle.
If you have an idea for a new feature file a feature request then assign it
to yourself to start work. This ensures others have clarity of new features
being added to the library.

Also, pull requests for adding, clarifying, or fixing typos in the
documentation are always welcome.


## Game Design Document
You might find the [game design document](https://fossxo.github.io/gdd/)
useful when working on this game. It provides the overall vision of the
game and describes a high level overview of the game's technical design.


## Tests
A goal of this project is to maintain excellent test coverage to ensure we
deliver a quality application. The game's full test suite can be run with:

```bash
cargo make test
```

If you are developing on Linux, the test coverage report is saved as
 `tarpaulin-report.html`.

### Unit Tests
When adding unit tests, they generally should conform to the following:

* There is a single `assert` statement in the test.
* There are no branches in the test; e.g. no `if`, `while`, or other such statements.
* The names following the format: unit of work **when** state under test **should** expected behavior.

See the existing unit tests for examples.


## Commits
Please try to keep commits small and containing a single logical change.

Consider these seven rules when writing a git commit message:

1. Separate subject from body with a blank line
2. Limit the subject line to 50 characters
3. Capitalize the subject line
4. Do not end the subject line with a period
5. Use the imperative mood in the subject line
6. Wrap the body at 72 characters
7. Use the body to explain what and why vs. how

The excellent [How to Write a Git Commit Message](https://chris.beams.io/posts/git-commit/)
guide provides additional details and examples for each of these items.


## Pull Requests
When the change you worked on is complete please ensure `cargo make pr-check`
runs without warnings or errors before sending a pull request.
