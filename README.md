# Rust CLI Template

This is a template repository which is well-suited for delivering clean, async CLIs through Rust.

## Table of Contents

- [Overview]()
- [Features]()
- [How to use]()
- [Contributing]()
- [License]()

## Overview

Writing a Rust CLI can be fun if you're doing it from scratch, but if you get ideas frequently but find them hard to execute without a proper structure at hand, this template is for you. It lets you focus on your idea rather than actually worrying about the structure.

This project actually takes inspiration from some existing projects like [mise](https://github.com/jdx/mise) and [cutler](https://github.com/cutlercli/cutler), and tries to combine the good practices into one single repository.

## Features

- Async by default; uses `async-trait` and `tokio` in conjunction.
- "Separation of concerns" for utilities, commands and I/O.
- Central logging with support for multiple types of categorization.
- Organized handling of global flags and command structs.
- A cute little interactive TUI function for letting the users confirm things with Y/n.
- `sudo` protection is preincluded if you need it.
- Pre-included CI/CD pipelines and Dependabot configuration.
... and more to come!

## How to use

Using is just as straightforward; simply clone the repository using the following command and you'll have it.

> [!NOTE]
> The project uses **Rust's 2024** edition, but you can change it in the [Cargo.toml](./Cargo.toml) file.

```bash
# clone
git clone https://github.com/hitblast/rust-cli-template.git

# check integrity
cd rust-cli-template && cargo check
```

### Manpage generation

This CLI template also comes with preincluded manpage generation. The output directory is `man/man1`.

```bash
cargo xtask manpage
```

## Contributing

Since it is a template repository, it does not have an explicit contribution guideline. But, if you have any ideas for what an ideal CLI should contain (written in Rust, obviously), then you're free to add to this project via a [pull request](https://github.blog/developer-skills/github/beginners-guide-to-github-creating-a-pull-request/)!

## License

This project has been licensed under [WTFPL](./LICENSE).
