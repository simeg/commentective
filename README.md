# detector [![Build Status](https://travis-ci.com/simeg/detector.svg?token=N26ztkyW6iXxAQwi2QWe&branch=master)](https://travis-ci.com/simeg/detector)
Find commented out code and comments in your project. Note that this tool
assumes that your code is syntactically correct.

|  Language  | Supported |
| ---------- | --------- |
| JavaScript |     ✔️    |
| Java       |     ✔️    |
| Rust       |     ✔️    |
| Python     |     ✔️    |
| C#         |     ✔️    |
| Bash/Shell |     ✔️    |
| PHP        |     ✔️    |
| Ruby       |     ✔️    |
| Go         |     ✔️    |
| Scala      |     ✔️    |
| CSS        |     ✔️    |
| HTML       |     ✔️    |
| C          |     ✔️    |
| C++        |     X     |

# Usage
`detector` is language agnostic, meaning you don't need to tell it what
language your files are written in. It will look at the extension of the
files and act accordingly.

```bash
$ detector [FILES]
```
