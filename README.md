# Ambassade

## What is Ambassade?

In short, Ambassade is an agnostic package manager. This means that Ambassade
excels at managing projects that are cross OS and/or cross language.

## Why Ambassade?

A lot of non-trivial projects require (technical) documentation. These
projects are usually too complex to understand at first glance and require
more than just one command to build. Most organizations have rules about how
documentation should be provided and in what format, as it makes clear how the
project's software works and how to use it. Ambassade tries to simplify this.
Just one `build` and one `run` command should suffice to build and run software.
Developers shouldn't have to know how every part of the software that they do
not directly use, works.

So Ambassade tries to solve an organizational problem: instead of having to
describe in the documentation how you as developer need to set up an ecosystem
that is required to successfully build and run your software, you just use
Ambassade to do it for you. This saves time, effort and frustration.

## How to use Ambassade?

The `ambassade help` command is enough to teach you the basics. For more
advanced usage of Ambassade, you will have to edit the config files.

## Getting Ambassade

### Download exectuable

You can download the executable from
[itch.io](https://helgade.itch.io/ambassade).

### Building from source

Use [Cargo](https://doc.rust-lang.org/stable/cargo/) to build Ambassade from
source:

```bash
cargo build # For debug version
cargo build --release # For release version
```

## Licence

See the [`LICENSE`](https://pagure.io/Ambassade/raw/master/f/doc/LICENSE) document for Ambassade's license.
