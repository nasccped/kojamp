Usage
=====

Here you'll find infos about subcommands and flags usage.

## Summary

- [help](#help)
- [new](#new)
  - [arguments and flags](#arguments-and-flags)

## Help

Will print a help panel of the program or the give subcommand.
It can be called as a subcommand or a flag too!

This:

```sh
kojamp --help
```

works the same as this:

```sh
kojamp help
```

Calling the help panel for a subcommand will work:

```sh
# this way
kojamp help subcommand_name

# or this way too
kojamp subcommand_name --help
```

## New

Create a Java/Kotlin project in a new folder.

### Arguments and flags

| name      | alias        | position index     | usage                                        | description                                                          | required |
| :-------- | :----------- | :----------------- | :------------------------------------------- | :------------------------------------------------------------------- | :------- |
| `name`    | **no alias** | 1                  | `<PROJECT_NAME>`                             | Specifies the project name                                           | Yes ✅   |
| `kind`    | `K`          | (called with flag) | `--kind [-K] <PROJECT_KIND>`                 | Specifies the project kind (Java or Kotlin)                          | Yes ✅   |
| `path`    | `P`          | (called with flag) | `--path [-P] <PROJECT_PATH>`                 | Specifies the path where the project will be created                 | No ❌    |
| `authors` | `A`          | (called with flag) | `--authors [-A] "<COMMA_SEPARATED_AUTHORS>"` | Specifies the authors for the project being created                  | No ❌    |
| `no-git`  | **no alias** | (called with flag) | `--no-git`                                   | Disable git repository initialization for the project being created  | No ❌    |
| `help`    | `-h`         | (called with flag) | `--help [-h]`                                | Print the help panel for the `new` subcommand                        | No ❌    |
