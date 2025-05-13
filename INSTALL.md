# Install

Welcome to the install page. Here you'll find ways to install this
program and it's requirements:

- by [cargo and crates.io 🦀](#cargo-and-cratesio-) _(recommended)_
- by [cargo, git and github 🐙](#cargo-git-and-github-)

> [!NOTE]
>
> Here are global requirements which you'll must have to build and
> run your kojamp projects:
> - `Java` development tools, such as ([java and javac](https://www.oracle.com/java/technologies/downloads/))
> - `Kotlin` compiler, aka [kotlinc](https://kotlinlang.org/docs/command-line.html#install-the-compiler)
>   _(only required for **Kotlin** projects)_

## Cargo and crates.io 🦀

### List of requirements

- [cargo package manager](https://www.rust-lang.org/tools/install) _`(^1.83.0)`_

### Step by step

1. Install using the `cargo` command line tool:

```sh
cargo install kojamp
```

2. Check if the program was successfully installed:

```sh
kojamp # a help panel is expected
```

## Cargo, Git and GitHub 🐙

### List of requirements

- [git](https://git-scm.com/)
- [cargo package manager](https://www.rust-lang.org/tools/install) _`(^1.83.0)`_

### Step by step

1. Clone the remote repository:

```sh
git clone --depth 1 https://github.com/nasccped/kojamp
```

2. Change directory to the cloned repo

```sh
cd kojamp
```

3. Install using `cargo`:

```sh
cargo install --path .
```

4. Check if the program was successfully installed:

```sh
kojamp # a help panel is expected
```
