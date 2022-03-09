# todo

[![CI](https://github.com/thekuwayama/todo/workflows/CI/badge.svg)](https://github.com/thekuwayama/todo/actions?workflow=CI)
[![MIT licensed](https://img.shields.io/badge/license-MIT-brightgreen.svg)](https://raw.githubusercontent.com/thekuwayama/todo/master/LICENSE.txt)
[![dependency status](https://deps.rs/repo/github/thekuwayama/todo/status.svg)](https://deps.rs/repo/github/thekuwayama/todo)

`todo` is a simple todo list command-line tool written in Rust.


## Install

You can install `todo` with the following:

```sh-session
$ cargo install --git https://github.com/thekuwayama/todo.git --branch main
```


## Usage

```sh-session
todo 0.1.0
simple command-line todo list

USAGE:
    todo <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add           add the task
    clear         clear todo list
    continue      continue todo list
    delete        delete the task
    done          done the task
    edit          edit the task description
    help          Print this message or the help of the given subcommand(s)
    list          show todo list
    record        record elapsed time
    report        report today's achievements
    swap          swap two tasks
    uncontinue    uncontinue todo list
    undone        undone the task
    unrecord      unrecord elapsed time
```

List todo

```sh-session
$ todo list
☐ 000: 朝起きる
☐ 001: 歯を磨く
☐ 002: シャワーを浴びる

```

Add new todo

```sh-session
$ todo add 散歩する
$ todo list
☐ 000: 朝起きる
☐ 001: 歯を磨く
☐ 002: シャワーを浴びる
☐ 003: 散歩する

```

Edit todo

```sh-session
$ todo edit 3 お水を一杯飲む
$ todo list
☐ 000: 朝起きる
☐ 001: 歯を磨く
☐ 002: シャワーを浴びる
☐ 003: お水を一杯飲む
```

Done todo

```sh-session
$ todo done 0
$ todo done 1
$ todo done 2
$ todo list
☑ 000: 朝起きる
☑ 001: 歯を磨く
☑ 002: シャワーを浴びる
☐ 003: お水を一杯飲む

```

Record elapsed time

```sh-session
$ todo record 0 0.1
$ todo record 1 0.1
$ todo record 2 0.5
$ todo list
☑ 000: 朝起きる (0.1)
☑ 001: 歯を磨く (0.1)
☑ 002: シャワーを浴びる (0.5)
☐ 003: お水を一杯飲む

```

Report today's achievements

```sh-session
$ todo report
## 2021/06/20 (0.7h)
### 進行中のタスク

### 完了済みのタスク
- 朝起きる (0.1h)
- 歯を磨く (0.1h)
- シャワーを浴びる (0.5h)

### その他、今週対応予定のタスク (金曜日は来週対応予定のタスク)
- お水を一杯飲む

### メモ、ぼやき

```

Continue todo list

```sh-session
$ todo continue
$ todo list
☐ 000: お水を一杯飲む

```


## Shell Completion

You can load the file to do the bash completion.

```sh-session
$ source todo.bash
```


## Note

`todo` is inspired by:

- https://github.com/todotxt/todo.txt-cli
- https://github.com/mattn/todo


## License

The CLI is available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT).
