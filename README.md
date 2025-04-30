<div align=center>

Kojamp
======

[![based on](https://img.shields.io/badge/based_on-rust-orange?)](https://www.rust-lang.org/)
[![clap version](https://img.shields.io/badge/clap_crate-v4.5.34-orange?)](https://docs.rs/clap/4.5.34/clap/index.html)
[![latest stable)](https://img.shields.io/github/v/tag/nasccped/kojamp?label=latest%20stable&color=31b564)](#)
[![license: Apache](https://img.shields.io/badge/License-Apache_2.0-blue?)](#)

</div>

This is Kojamp, a Java/Kotlin project manager tool 📦

## Requirements

This tool targets Java/Kotlin projects and requires:

<div align="center">

| Tool(s)           | Description                       | Link                                                                                 | Required?                                           |
| :---------------- | :-------------------------------- | :----------------------------------------------------------------------------------- | :-------------------------------------------------- |
| `rustc` + `cargo` | Rust compiler and package manager | [rust-lang.org](https://www.rust-lang.org)                                           | Yes ✅ (source is in Rust)                           |
| `javac` + `java`  | Java compiler and runtime         | [oracle.com](https://www.oracle.com/java/technologies/downloads/)                    | No ❌ (required only for Java compilation/execution) |
| `kotlinc`         | Kotlin compiler                   | [kotlinlang.org](https://kotlinlang.org/docs/command-line.html#install-the-compiler) | No ❌ (same as Java)                                 |


</div>

## Installation

> [!IMPORTANT]
>
> Requires [Git](https://git-scm.com/)

1. Clone the repository ⬇️

```sh
git clone --depth 1 https://github.com/nasccped/kojamp
rm -rf kojamp/.git
# If you're at Windows Powershell, use this:
# Remove-Item kojamp/.git -Recurse -Force
```

2. Build and install 🪛

```sh
cd kojamp
cargo install --path .
```

3. Verify installation 🔭

```sh
kojamp --help
```

> [!CAUTION]
>
> `Kojamp` is under active development!
>
> For current progress, see
> [WARN_MESSAGE.md](https://github.com/nasccped/kojamp/blob/main/WARN_MESSAGE.md)
> 😉

<div align=center>

[![license: Apache](https://img.shields.io/badge/License-Apache_2.0-blue?)](#)

</div>
