# todo

[![CI](https://github.com/thekuwayama/todo/workflows/CI/badge.svg)](https://github.com/thekuwayama/todo/actions?workflow=CI)
[![MIT licensed](https://img.shields.io/badge/license-MIT-brightgreen.svg)](https://raw.githubusercontent.com/thekuwayama/todo/master/LICENSE.txt)

`todo` is a simple todo list command-line tool written in Rust.


## Usage

```bash
$ todo
todo 0.1.0
simple todo list command-line tool

USAGE:
    todo [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add         add the task
    clear       clear todo list
    delete      delete the task
    done        done the task
    help        Prints this message or the help of the given subcommand(s)
    list        show todo list
    record      record elapsed time
    report      report today's achievements
    swap        swap two tasks
    undone      undone the task
    unrecord    unrecord elapsed time
```


## License

The CLI is available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT).
