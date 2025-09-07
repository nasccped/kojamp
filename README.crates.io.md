<div align=center>

Kojamp
======

[![license: Apache](https://img.shields.io/badge/License-Apache_2.0-blue?)](#)

</div>

> [!CAUTION]
>
> This project is no longer maintained. Try using
> [jsmoke](https://github.com/nasccped/jsmoke) instead!
>
> The official README is located at it's own repository:
> https://github.com/nasccped/kojamp

This is **Kojamp**, a Java and Kotlin project manager tool 📦

## Requirements

- [rust tools](https://www.rust-lang.org/) (compiler + package
  manager)
- [java tools](https://www.oracle.com/java/technologies/downloads/)
  (`java`, `javac`, ...)
- [kotlin compiler](https://kotlinlang.org/docs/command-line.html#install-the-compiler)
  (if building kotlin projects)

## Installation

The following command will install **Kojamp** at your machine:
```sh
cargo install kojamp
```

And now, you can test if the program runs:
```sh
kojamp # A help panel is expected
```

## Usage

### Create a new project

The command bellow will create a new directory _(code-dude)_ with a
java project:
```sh
kojamp new CodeDude --kind java
```

### Build the project's bytecode

Just like others project managers (`maven`, `gradle`, ...), you can
use the `build` (or `b`) subcommand to compile your code:

```sh
kojamp build
```

It will place the bytecode at `out` directory (if compilation
success).

### Run the bytecode

The command bellow will run your project's bytecode output:

```sh
kojamp run
```

## Inspirations

- **[bacon](https://github.com/Canop/bacon) by [Denys Séguret](https://github.com/Canop):**
  good crate documentation
- **[jargo](https://github.com/Marlon-Sbardelatti/jargo) by [Marlon Sbardelatti](https://github.com/Marlon-Sbardelatti):**
  similar project

## License

This project is under the
[Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) license.
