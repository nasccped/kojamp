<div align=center>

Kojamp
======

[![license: Apache](https://img.shields.io/badge/License-Apache_2.0-blue?)](#)

</div>

> ⚠️ This is a shorter README version designed to be displayed only at
> crates.io.
>
> Consider going to the
> [official GitHub repository](https://github.com/nasccped/kojamp).

This is **Kojamp**, a Java and Kotlin project manager tool 📦

## Installation

The following command will install **Kojamp** at your machine ⬇️

```sh
cargo install kojamp
```

And now, you can test if the program runs 🔬

```sh
kojamp # A help panel is expected
```

## Usage

### Create a new project

The commad bellow will create the `CodeDude` java project at code-dude
directory (but you still can specify a directory destination by using
the `--path` | `-p` flag followed by the path name 🤓☝️)

```sh
kojamp new CodeDude --kind java
```

### Build the project's bytecode

Works the same as `maven`, `gradle` and other project managers. It
will take the java/kotlin files at src dir, compile it and drop the
bytecode at `out` directory (you can use `b` as alias for this
subcommand).

```sh
kojamp build
```

> ⚠️ The program is undergoing a development stage. Not all features
> have been implemented. Check the
> [official repo](https://github.com/nasccped/kojamp)!

## Inspirations

- **[bacon](https://github.com/Canop/bacon) by [Denys Séguret](https://github.com/Canop):**
  good crate documentation 🐷
- **[jargo](https://github.com/Marlon-Sbardelatti/jargo) by [Marlon Sbardelatti](https://github.com/Marlon-Sbardelatti):**
  similar project 🏝️

## License

This project is under the
[Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) license.
