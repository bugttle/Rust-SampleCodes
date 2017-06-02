# Rust-SampleCodes

## Installation
### rustup
* Use `rustup` instead of `homebrew`. You will fail to install `rustfmt` if used `homebrew`.
  * https://www.rustup.rs/

* To install
 ```
$ curl https://sh.rustup.rs -sSf | sh
 ```

* To create a new project
 ```
 $ cargo new PROJECT_NAME --bin
 ```

### rustfmt
* To install
```
$ cargo install rustfmt
```

* To run on a cargo project
```
$ cargo fmt
```

* Configuring rustfmt
  * https://github.com/rust-lang-nursery/rustfmt/blob/master/Configurations.md
