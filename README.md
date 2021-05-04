![Rust](https://github.com/kamiyaa/tsuchita/workflows/Rust/badge.svg)

# tsuchita

A client-server notification center for dbus desktop notifications.

Specifically `org.freedesktop.Notifications` dbus messages.

## Motivation

I just want to be able to see missed notifications on a window manager.

## Dependencies

- [cargo](https://github.com/rust-lang/cargo/)
- [rustc](https://www.rust-lang.org/)
- [libdbus](https://www.freedesktop.org/wiki/Software/dbus/)

Also see [Cargo.toml](Cargo.toml)

## Building

```
~$ cargo build
```

## Installation

#### For single user

```
~$ cargo install --path=. --force
```

#### System wide

```
~# cargo install --path=. --force --root=/usr/local     # /usr also works
```

## Usage

To start the server and listen for dbus notifications:
```
~ $ tsuchita-server
```

To start a terminal client that reads notifications from the server
```
~ $ tsuchita
```

## Configuration

Place config files inside `$XDG_CONFIG_HOME/tsuchita` (usually `$HOME/.config/tsuchita/` for GNU/Linux).

tsuchita can currently be configured using the following files:

#### [tsuchita.toml](config/tsuchita.toml)

- general configurations

#### [keymap.toml](/config/keymap.toml)

- for keybindings, please take a look at [src/client/util/key_mapping.rs](/src/util/key_mapping.rs) for non-printable keys
- for commands, please take a look at [src/client/commands/commands.rs](/src/commands/command.rs)

#### [theme.toml](/config/theme.toml)

- color customizations

## Contributing

Please create a pull request :)

## Features/Bugs

Please create an issue :)

## TODOs

### Server
- [ ] database store
- [ ] CRUD

### Terminal Client
- [x] tui interface
- [ ] CRUD

### GUI Client
TODO
