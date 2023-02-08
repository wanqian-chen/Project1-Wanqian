# Individual project1

@TODO: Math tool

This project can calculate the statistics of a given array of numbers. The statistics include the mean, median, mode, and standard deviation. Also, it can calculate the chi-square of the given array of numbers.

It can be done by using in-line commands.

Also, I deployed this project on https://iesjijhu3j.us-east-1.awsapprunner.com/. You can use this link to test the project.

Feel free to check them!

<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-refresh-toc -->
**Table of Contents**

- [Individual project1](#individual-project1)
    - [Requirements](#requirements)
    - [Usage](#usage)
        - [Installation](#installation)
        - [Usage](#usage-1)
        - [Links](#links)
    - [References](#references)

## Requirements

* [Rust](https://www.rust-lang.org/en-US/install.html)

## Usage

### Installation

```bash
$ cargo install --git
```

### Usage

For in-line commands:

```bash
$ cargo run --{function-name} --{argument} {value}
```

For web server:

```bash
$ cargo run -- --help
```

### Links

https://iesjijhu3j.us-east-1.awsapprunner.com/  
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

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
