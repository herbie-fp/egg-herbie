# An ffi interface to mwillsey/egg for use in herbie

[![Build Status](https://travis-ci.org/oflatt/egg-herbie.svg?branch=master)](https://travis-ci.org/oflatt/egg-herbie)


## Developing

It's written in [Rust](https://www.rust-lang.org/).
Typically, you install Rust using [`rustup`](https://www.rust-lang.org/tools/install).

Before committing/pushing, run `make` to check tests and formatting.
To use as a package for herbie, run raco pkg install "this folder".