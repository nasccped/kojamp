<div align=center>

Kojamp
======

[![based on](https://img.shields.io/badge/based_on-rust-orange?)](https://www.rust-lang.org/)
[![clap version](https://img.shields.io/badge/clap_crate-v4.5.34-orange?)](https://docs.rs/clap/4.5.34/clap/index.html)
[![latest stable)](https://img.shields.io/github/v/tag/nasccped/kojamp?label=latest%20stable&color=31b564)](#)
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

1. Clone the repository 🏷️

```sh
git clone --depth 1 https://github.com/nasccped/kojamp
```

2. Build and install 🪛

```sh
cargo install --path kojamp
```

3. Verify installation 🔭

```sh
kojamp --help
```

## Usage

> [!WARNING]
>
> Not all operations have been implemented. Check the
> [WARN_MESSAGE.md](https://github.com/nasccped/kojamp/blob/main/WARN_MESSAGE.md).

### Create a project in a new folder

We can use the `new` subcommand, like this:

```sh
kojamp new ProjectExample --kind java
```

This will create a new Java project called `ProjectExample`. We can
also create a Kotlin project and specify a path by using:

```sh
kojamp new OtherExample --kind kotlin --path specified-path
```

## License

This project is under the
[Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) license.

<div align=center>

[![license: Apache](https://img.shields.io/badge/License-Apache_2.0-blue?)](#)

</div>
