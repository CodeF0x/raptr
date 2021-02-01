# Welcome to raptr

raptr is a minimalistic and opinionated blogging engine. It's fast, easy to set-up and easy to use. 

---

## ~ Features ~

* use themes made by the community or create your own
* edit and tweak existing themes
* fast and easy to use with only the bare minimum of commands
* outputs web-ready websites with all required assets -- just put it on your web server and you're ready to go
* quickly create posts and write them in markdown
* raptr comes as a standalone -- no dependencies like Java, Go, Node.js, you name it

## ~ Installation ~

Right now you need to compile it yourself -- sorry! :( But I'll provide binaries in the near future!

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
