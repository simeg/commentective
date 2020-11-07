# Changelog

## 0.8.1
* Bug fix: Clap updated the API and needed updating

## 0.8.0
* Add flag to analyze files as specified language (-l / --lang)

## 0.7.0
* Add flag to also output the code with the comment (-c / --code)

## 0.6.0
* Silently ignore directories
* Include the file name for unsupported files
* Big code refactor
* Use Rust 2018 edition

## 0.5.0
* Add flag to ignore outputting files without any comments in them

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
