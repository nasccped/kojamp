<div align=center>

Kojamp
======

[![built in](https://img.shields.io/badge/built_in-rust-orange?)](https://www.rust-lang.org/)
[![crates io](https://img.shields.io/crates/v/kojamp.svg)](https://crates.io/crates/kojamp)
[![license: Apache](https://img.shields.io/badge/License-Apache_2.0-blue?)](#)

</div>

This is Kojamp, a Java/Kotlin project manager tool 📦

> [!NOTE]
>
> On this page you'll find the basics of the program. If you want to
> check the app guide more deeply, you can read the official
> repository files:
>
> - [INSTALL.md](https://github.com/nasccped/kojamp/blob/main/INSTALL.md):
> important info about the requirements and installation ⬇️
> - [USAGE.md](https://github.com/nasccped/kojamp/blob/main/USAGE.md):
> important info about the program usage 👨‍💻
>
> ---
>
> The project is under active development. You can check the updates
> and the warnings by reading the
> [WARN_MESSAGE.md](https://github.com/nasccped/kojamp/blob/main/WARN_MESSAGE.md)
> 😉

## Installation

The following command will install **Kojamp** at your machine ⬇️

```sh
cargo install kojamp
```

> [!TIP]
>
> [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html)
> program is required to run the program above, but you can find
> other ways of installation at
> [INSTALL.md](https://github.com/nasccped/kojamp/blob/main/INSTALL.md).

And now, you can test if the program runs 🔬

```sh
kojamp # A help panel is expected
```

## Usage

> [!WARNING]
>
> Not all operations have been implemented. Check the
> [WARN_MESSAGE.md](https://github.com/nasccped/kojamp/blob/main/WARN_MESSAGE.md).

### Create a new project

```sh
kojamp new CodeDude --kind java
```

This will create a new Java project called `CodeDude`. The project
will be inside an auto-generated directory called `code-dude`, but
you can still specify a path by using the `--path` flag.

> [!NOTE]
>
> The subcommand(s) above are just a sample of the `kojamp` features.
> You can find all features guide at
> [USAGE.md](https://github.com/nasccped/kojamp/blob/main/USAGE.md).

## License

This project is under the
[Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) license.

<div align=center>

[![license: Apache](https://img.shields.io/badge/License-Apache_2.0-blue?)](#)

</div>
