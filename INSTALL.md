# Install

Welcome to the install page. Here you'll find ways to install this
program and it's requirements!

## Requirements

This tool targets Java/Kotlin projects and requires:

1. [`rustc`](https://www.rust-lang.org/tools/install) and
   [`cargo`](https://www.rust-lang.org/tools/install), tools to
   compile Rust source code + it's dependencies;
2. [`javac`](https://www.oracle.com/java/technologies/downloads/) and
   [`java`](https://www.oracle.com/java/technologies/downloads/),
   Java tools to compile and run Java source code;
3. [`kotlinc`](https://kotlinlang.org/docs/command-line.html#install-the-compiler),
   Kotlin compiler (required only if you're building Kotlin
   projects).

## Installation

Currently, there are the following options for installing this
project:

- 📦 [git and cargo](#git-and-cargo-)

### Git and Cargo 📦

> [!IMPORTANT]
> This install way requires [`git`](https://git-scm.com/) too!

1. Clone the repository + remove git folder:

```sh
git clone --depth 1 https://github.com/nasccped/kojamp
rm -rf kojamp/.git
# You should use the command bellow if you're at Windows Powershell
Remove-Item kojamp/.git -Recurse -Force
```

2. Install by using `cargo`:

```sh
cargo install --path kojamp
```

3. Test if the program was successfully installed:

```sh
kojamp --help
```
