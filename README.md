# Eshu

TODO:

- Consider ArgosCI integration
- Consider needed dependencies in `Cargo.toml`

A zero-dependency Rust library for building robust CLI tools with automatic help and man-page generation.

It follows my "All code written by me or part of rust's standard library and libc" philosophy.
You can learn more about that [here](https://blog.xqhare.net/posts/why_solve_problems/).

## Features

- _**No dependencies**_: All code is written by me or part of std.

## Naming

As with all my projects, `Eshu` is named after an ancient deity.
Learn more about my naming scheme [here](https://blog.xqhare.net/posts/explaining_the_pantheon/).

An East African divine spirit of communication and language, acting as a messenger between humans and the deities.

## Quick Reference

```bash
$ tool_name [-a] [-bC] [-d required_arg] [-e [optional_arg]] [-f=required_arg] [-g[=optional_arg]] [-h arg0] [-h argN] [-i key=value] [--long-flags] [--long-flag[=optional_arg]] [--long-flag=required_arg] [--flag_with=key=value] [--] [optional_positional_args] [subcommand] [subcommand_args_and_flags]
```

- Short Flags (Upper and Lowercase)
- Long Flags
- Key Value Pairs
- Named Key Value Pairs
- Flags with optional or required arguments
- End-Of-Flags Marker
- Positional Arguments
- Subcommands

### Always Available Flags

- `--help` and `-h`: Prints the help message
- `--version`: Prints the version number (Note: `-v` and `-V` are free to be used for custom flags)

## Supported CLI Syntax

It is important to know that there is no formalised syntax agreed upon for CLI tools.
If you want to dive into the details, I have written a blog post [A Guide to the Syntax of CLI Tooling: Modern Problems Require Old Solutions](https://blog.xqhare.net/posts/cli_modern_problems_old_solutions/).

Please note, I use the term "flag" to refer to all elements that start with a dash `-` or two dashes `--`.
The term "key value pair" refers to all elements that contain at least one equal sign `=`. Key value pairs may never have more than two equal signs. If a pair contains two equal signs,it is called a named key value pair.
The term "argument" refers to all elements that are passed to a specific flag or command. These may be required or optional.

In general terms the syntax offered by `Eshu` is:

```bash
$ tool_name [-a] [-bC] [-d required_arg] [-e [optional_arg]] [-f=required_arg] [-g[=optional_arg]] [-h arg0] [-h argN] [-i key=value] [--long-flags] [--long-flag[=optional_arg]] [--long-flag=required_arg] [--flag_with=key=value] [--] [optional_positional_args] [subcommand] [subcommand_args]
```

In other terms, `Eshu` supports the POSIX and GNU syntax in general, along with a few modern best practices.

## Usage

### Best Practices Using `Eshu`

`Eshu` is best used by following the general best practices for CLI tools.
You can find some [here](https://blog.xqhare.net/posts/cli_modern_problems_old_solutions/#further-reading).

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
