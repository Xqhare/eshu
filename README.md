# Eshu

TODO:

- Consider ArgosCI integration

A zero-dependency Rust library for building robust CLI tools with automatic help and man-page generation.

It follows my "All code written by me or part of rust's standard library and libc" philosophy.
You can learn more about that [here](https://blog.xqhare.net/posts/why_solve_problems/).

## Roadmap

- Rework help generation to use correctly use unicode characters for width, instead of assuming ASCII

### Open Until Considered Stable

- [ ] Man-page generation
- [ ] Error handling refactoring
- [ ] Doc
    - [ ] Examples
    - [ ] Readme
- [ ] Tests

## Features

- _**No dependencies**_: All code is written by me or part of std.
- _**XffValue Integration**_: Easy casting of returned values into `XffValues`.

## Naming

As with all my projects, `Eshu` is named after an ancient deity.
Learn more about my naming scheme [here](https://blog.xqhare.net/posts/explaining_the_pantheon/).

`Eshu` is an East African divine spirit of communication and language, acting as a messenger between humans and the deities.

## Quick Reference

```bash
$ tool_name [-a] [-bC] [-d required_arg] [-e [optional_arg]] [-f=required_arg] [-g[=optional_arg]] [-h arg0] [-h argN] [-i key=value] [--long-flags] [--long-flag[=optional_arg]] [--long-flag=required_arg] [--flag_with=key=value] [-- optional_positional_args] [subcommand] [subcommand_args_and_flags]
```

- Short Flags (Upper and Lowercase)
- Long Flags
- Grouped Flags
- Key Value Pairs
- Named Key Value Pairs
- Flags with optional or required arguments
- End-Of-Flags Marker (`--`)
- Positional Arguments
- Subcommands

> [!NOTE]
> Partial long flags are supported as long as they are unique from other long flags.
>
> For example, `--long` and `--long-flag` are both valid ways to call `--long-flag` as long as there is no other flag beginning with `--long`.

> [!note]
> All flags accepting an argument can store any number of arguments.
> 
> For example, `--long-flag arg0 arg1 arg2` is equivalent to `--long-flag arg0 --long-flag arg1 --long-flag arg2`.

> [!note]
> A Flag accepting an argument can be part of grouped flags if only one is used and placed at the end of the group.
>
> For example, `-ab -C=Argument` can be written as `-abC=Argument`.

### Always Available / in-Use Flags

These flags will only print to `stdout` and exit.

- `--help` and `-h`: Prints the help message
- `--version`: Prints the version number (Note: `-v` and `-V` are free to be used for custom flags)

> [!NOTE] 
> These flags cannot be overwritten by other flags. Usage of them is mandatory and automatic.

### Flag Name Definitions

`Eshu` encourages you to make use of flag names defined by the [GNU Coding Standards](https://web.mit.edu/gnu/doc/html/standards_18.html).

## Supported CLI Syntax

It is important to know that there is no formalised syntax agreed upon for CLI tools.
If you want to dive into the details, I have written a blog post [A Guide to the Syntax of CLI Tooling: Modern Problems Require Old Solutions](https://blog.xqhare.net/posts/cli_modern_problems_old_solutions/).

Please note, I use the term "flag" to refer to all elements that start with a dash `-` or two dashes `--`.
The term "key value pair" refers to all elements that contain at least one equal sign `=`. Key value pairs may never have more than two equal signs. A pair may only contain two equal signs if it is an attached value.
The term "argument" refers to all elements that are passed to a specific flag or command. These may be required or optional.

In general terms the syntax offered by `Eshu` is:

```bash
$ tool_name [-a] [-bC] [-d required_arg] [-e [optional_arg]] [-f=required_arg] [-g[=optional_arg]] [-h arg0] [-h argN] [-i key=value] [--long-flags] [--long-flag[=optional_arg]] [--long-flag=required_arg] [--flag_with=key=value] [--] [optional_positional_args] [subcommand] [subcommand_args]
```

`Eshu` also supports partial long flags, as long as they are unique from other long flags.

A flag accepting an argument can store any number of arguments.
Also, a flag accepting an argument can be part of grouped flags if only one is used and placed at the end of the group.

In other terms, `Eshu` supports the POSIX and GNU syntax in general, along with a few modern best practices.

## Usage

### Best Practices Using `Eshu`

`Eshu` is best used by following the general best practices for CLI tools.
You can find some [here](https://blog.xqhare.net/posts/cli_modern_problems_old_solutions/#further-reading).

`Eshu` encourages you to make use of flag names defined by the [GNU Coding Standards](https://web.mit.edu/gnu/doc/html/standards_18.html).

### Importing

Add the following to your `Cargo.toml`:

```toml
[dependencies]
eshu = { git = "https://github.com/xqhare/eshu" }
```

### Example

```rust

```

## License

`Eshu` is distributed under the [MIT](https://github.com/xqhare/eshu/blob/master/LICENSE) license.

## Contributing

See [CONTRIBUTING](https://github.com/xqhare/eshu/blob/master/CONTRIBUTING.md) for contribution guidelines.
