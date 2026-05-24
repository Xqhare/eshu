# Eshu

TODO:

- Consider ArgosCI integration
- Consider needed dependencies in `Cargo.toml`

A zero-dependency Rust library for building robust CLI tools with automatic help and man-page generation.

It follows my "All code written by me or part of rust's standard library and libc" philosophy.
You can learn more about that [here](https://blog.xqhare.net/posts/why_solve_problems/).

## Features

- _**No dependencies**_: All code is written by me or part of std.

## Environment

Eshu expects the environment to provide:

- `ls` UNIX command

## Naming

As with all my projects, Eshu is named after an ancient deity.
Learn more about my naming scheme [here](https://blog.xqhare.net/posts/explaining_the_pantheon/).

An East African divine spirit of communication and language, acting as a messanger between humans and the deities.

## Usage

### Importing

Add the following to your `Cargo.toml`:

```toml
[dependencies]
Eshu = { git = "https://github.com/xqhare/eshu" }
```

### Example

```rust

```

## License

Eshu is distributed under the [MIT](https://github.com/xqhare/eshu/blob/master/LICENSE) license.

## Contributing

See [CONTRIBUTING](https://github.com/xqhare/eshu/blob/master/CONTRIBUTING.md) for contribution guidelines.
