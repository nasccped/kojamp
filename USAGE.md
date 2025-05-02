Usage
=====

Here you find infos about subcommand and flags usage.

## Subcommand summary

- [subcommands](#subcommands)
  - [help](#help-subcommandflag)
  - [new](#new-subcommand)

## Subcommands

### Help subcommand/flag

This will print a help panel of the program or a subcommand.
It can be called as a subcommand or a flag too, for example:

this:

```sh
kojamp --help
```

works the same as this:

```sh
kojamp help
```

---

You can also call a subcommand help panel, like:

```sh
kojamp <SUBCOMMAND_NAME> --help
```

### New subcommand

This will create a Java/Kotlin project in a new folder and have the
following args/options:

| name      | alias        | position index     | usage                                        | description                                                          | required |
| :-------- | :----------- | :----------------- | :------------------------------------------- | :------------------------------------------------------------------- | :------- |
| `name`    | **no alias** | 1                  | `<PROJECT_NAME>`                             | Specifies the project name                                           | Yes ✅   |
| `kind`    | `K`          | (called with flag) | `--kind [-K] <PROJECT_KIND>`                 | Specifies the project kind (Java or Kotlin)                          | Yes ✅   |
| `path`    | `P`          | (called with flag) | `--path [-P] <PROJECT_PATH>`                 | Specifies the path where the project will be created                 | No ❌    |
| `authors` | `A`          | (called with flag) | `--authors [-A] "<COMMA_SEPARATED_AUTHORS>"` | Specifies the authors for the project being created                  | No ❌    |
| `no-git`  | **no alias** | (called with flag) | `--no-git`                                   | Disable git repository initialization for the project being created  | No ❌    |
| `help`    | `-h`         | (called with flag) | `--help [-h]`                                | Print the help panel for the `new` subcommand                        | No ❌    |
