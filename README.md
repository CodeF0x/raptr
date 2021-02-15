# Welcome to raptr

raptr is a minimal, static site generator and blogging-engine. It's fast, easy to set-up and easy to use.

---

## ~ Features ~

- use themes made by the community or create your own
- edit and tweak existing themes
- fast and easy to use with only the bare minimum of commands
- outputs web-ready websites with all required assets -- just put it on your web server and you're ready to go
- quickly create posts and write them in markdown
- raptr comes as a small standalone for any system

## ~ Installation ~

You need <a href="https://git-scm.com/">Git</a>, which you probably already have, and <a href="https://doc.rust-lang.org/cargo/">Cargo</a> which is usally pretty easy to install.
Once you've got both, run

```shell
cargo install raptr
```

in your terminal and you're done!

For other methods, see <a href="https://github.com/CodeF0x/raptr/wiki/Installation">"Install raptr"</a> in the official wiki.

## ~ Usage ~

A quick start guide is available <a href="https://github.com/CodeF0x/raptr/wiki/Quick-start">here</a>.

```shell
$ raptr --help
raptr 0.1.0
Tobias "CodeF0x" Oettl <contact@codef0x.dev>
An opinionated blogging engine

USAGE:
    raptr [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Shows detailed errors and logging messages

OPTIONS:
    -d, --draft <DRAFT_NAME>            Creates a new draft
    -n, --new <PROJECT_NAME>            Creates a new project
    -p, --publish <OUTPUT_DIRECTORY>    Renders your drafts to web-ready html files [default: output]
```
