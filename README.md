# Individual project1

![Rustc Version][rustc-image]
[![Crates.io Version][crate-image]][crate-link]
[![Docs.rs Version][docs-image]][docs-link]
[![dependency status][deps-image]][deps-link]

@TODO: Math tool

This project can calculate the statistics of a given array of numbers. The statistics include the mean, median, mode, and standard deviation. Also, it can calculate the chi-square of the given array of numbers.

I deployed this project on https://iesjijhu3j.us-east-1.awsapprunner.com/. You can use this link to test the project.

The functions contain the following:
A. / that returns a hello world message
B. /hello/{name} that returns a hello message
C. /delete_zero/{v} that returns a vector without zero
D. /coin/{probability} that returns 1 or 0
E. /mean/{v} that returns the mean of a list
F. /median/{v} that returns the median of a list
G. /mode/{v} that returns the mode of a list
H. /variance/{v} that returns the variance of a list
I. /std/{v} that returns the standard deviation of a list
J. /chi_square/{v} that returns the chi-square of a list

Feel free to check them!

<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-refresh-toc -->
**Table of Contents**

- [`Individual project1`](#individual-project1)
    - [The Pitch](#the-pitch)
    - [The Anit-Pitch](#the-anit-pitch)
- [Installation](#installation)
    - [Compile from Source](#compile-from-source)
- [Usage](#usage)
    - [Command Line Interface](#command-line-interface)
- [License](#license)
    - [Contribution](#contribution)

<!-- markdown-toc end -->

## The Pitch

@TODO: pitch

## The Anit-Pitch

@TODO: anti-pitch

# Installation

`{{crate_name}}` is a single binary that must be placed somewhere in your
`$PATH`. One can either download 64-bit Linux binaries from [the Release Page](https://github.com/kbknapp/iptables_exporter/releases)
or one can also compile from source.

## Compile from Source

Ensure you have a [Rust toolchain installed](https://rustup.rs). Some of the
dependencies also require `gcc` to be installed.

```
$ git clone https://github.com/kbknapp/{{project-name}}
$ cd {{project-name}}
$ cargo build --release
$ sudo cp target/release/{{crate_name}} /usr/local/bin/
```

# Usage

## Command Line Interface

```
@TODO: cli usage
```

# License

This crate is licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)

[rustc-image]: https://img.shields.io/badge/rustc-1.53+-blue.svg
[crate-image]: https://img.shields.io/crates/v/{{project-name}}.svg
[crate-link]: https://crates.io/crates/{{project-name}}
[docs-image]: https://docs.rs/{{project-name}}/badge.svg
[docs-link]: https://docs.rs/{{project-name}}
[deps-image]: https://deps.rs/repo/github/kbknapp/{{project-name}}/status.svg
[deps-link]: https://deps.rs/repo/github/kbknapp/{{project-name}}