# Aoc Initializer

Created to easily create a template solution with some boilerplate for aoc. Written in `Rust` since that is what I am using for `aoc 2022`.

## Prereqs

- Get your AOC session id via the cookie and export the environment variable `AOC_SESSION_ID` with it's value
- Have cargo setup

## Setup

- Pull down the repo and cd into it
- `cargo build --release`
- place build in `/usr/bin` or add an alias to run the aoc-initializer in your shell
- go into the directory you wish to generate the boiler plate in
- run `aoc-initializer -d 5`
