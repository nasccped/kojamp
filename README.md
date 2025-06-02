<div align=center>

Kojamp
======

[![crates io](https://img.shields.io/crates/v/kojamp.svg)](https://crates.io/crates/kojamp)
[![license: Apache](https://img.shields.io/badge/License-Apache_2.0-blue?)](#)

</div>

This is **Kojamp**, a Java and Kotlin project manager tool 📦

> [!NOTE]
>
> This is just a short overview about the Kojamp project. To see all
> detailed info, check:
>   - [`INSTALL.md`](https://github.com/nasccped/kojamp/blob/main/INSTALL.md), the installation guide 
>   - [`USAGE.md`](https://github.com/nasccped/kojamp/blob/main/USAGE.md), the usage guide

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

The command bellow will create the `CreepShow` java project at
creep-show directory, but you still can specify a directory
destination by using the `--path` | `-p` flag followed by the path
name.

```sh
kojamp new CodeDude --kind java
```

> [!TIP]
>
> By using `--kind java` flag/arg, a Java project will be created but
> you can also use the `k` as `kind` alias and pass kotlin as value.

### Build the project's bytecode

Just like others project managers (`maven`, `gradle`, ...), you can
use the `build` (or `b`) to compile your code:

```sh
kojamp build
```

This will take all your source code files (`.java` or `.kt`,
depending on your project kind) compile and put it at `out` dir if
**Java** project, else, `out/<PROJECT_NAME>.jar` if **Kotlin**
project.

## Inspirations

Others projects that helped me during the development 🫂

- **[bacon](https://github.com/Canop/bacon) by [Denys Séguret](https://github.com/Canop):**
  good crate documentation 🐷
- **[jargo](https://github.com/Marlon-Sbardelatti/jargo) by [Marlon Sbardelatti](https://github.com/Marlon-Sbardelatti):**
  similar project 🏝️

## License

This project is under the
[Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) license.
