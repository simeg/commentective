# commentective [![Build Status](https://travis-ci.com/simeg/commentective.svg?token=N26ztkyW6iXxAQwi2QWe&branch=master)](https://travis-ci.com/simeg/commentective) [![codecov](https://codecov.io/gh/simeg/commentective/branch/master/graph/badge.svg)](https://codecov.io/gh/simeg/commentective)
`commentective` (word play on _comment detective_) is a CLI tool that locates commented out code and
 comments in your project. Note that this tool assumes that your code is syntactically correct.

It will find single line comments and multi line comments. It supports a number of languages. If you
find a case that `commentective` does not support, please
[submit an issue](https://github.com/simeg/commentective/issues/new).


# Usage
`commentective` is language agnostic, meaning you don't need to tell it what language your files are 
written in. It will look at the extension of the files and act accordingly.

```bash
$ commentective [FILES]
```


# Supported languages

|  Language  | Supported |
| ---------- | --------- |
| Bash/Shell |     ✔️    |
| C          |     ✔️    |
| C#         |     ✔️    |
| C++        |     ✔️    |
| CSS        |     ✔️    |
| Go         |     ✔️    |
| HTML       |     ✔️    |
| Java       |     ✔️    |
| JavaScript |     ✔️    |
| Lua        |     ✔️    |
| PHP        |     ✔️    |
| Python     |     ✔️    |
| Ruby       |     ✔️    |
| Rust       |     ✔️    |
| Scala      |     ✔️    |

Got a request for a language? 
[Submit an issue!](https://github.com/simeg/commentective/issues/new)


# Installation
```bash
$ cargo install commentective
```


# Contribute :zap:

Want to contribute to this project? A backlog is kept in the
[issues](https://github.com/simeg/commentective/issues), have a look!
