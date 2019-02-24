# Changelog

## 0.4.0
* Run evaluation of files in parallel, increasing speed ~50%

## 0.3.0

* The language Lua is now supported (`.lua`)
* If something goes wrong in the execution - status `1` is returned
* Flags:
  * `--short` to output `file.ext:line` to make parsing easier
  * `--extension` to specify what type of file to analyze. Useful when reading a lot of files but you're only really interested in one type.

(thanks @tversteeg)

## 0.2.0

* Initial release
