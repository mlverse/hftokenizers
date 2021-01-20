# HuggingFace tokenizers from R

[![R build status](https://github.com/mlverse/hftokenizers/workflows/R-CMD-check/badge.svg)](https://github.com/mlverse/hftokenizers/actions) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> This is an experimental project binding HuggingFace [tokenizers](https://github.com/huggingface/tokenizers) Rust library to R using the [extendr](https://github.com/extendr/extendr) project. Do **not** use for anything meaninful yet.

## Installation

This repository uses the [helloextendr template](https://github.com/extendr/helloextendr).

Before you can install this package, you need to install a working Rust toolchain. We recommend using [rustup.](https://rustup.rs/)

On Windows, you'll also have to add the `i686-pc-windows-gnu` and `x86_64-pc-windows-gnu` targets:

    rustup target add x86_64-pc-windows-gnu
    rustup target add i686-pc-windows-gnu

Once Rust is working, you can install this package via:

``` {.r}
remotes::install_github("mlverse/hftokenizers")
```

## Small example

Here's a quick demo of what you can do with `hftokenizers`:

``` {.r}
library(hftokenizers)

download.file(
  "https://raw.githubusercontent.com/mlverse/hftokenizers/main/tests/testthat/assets/small.txt",
  "small.txt"
)

tokenizer$
  new(models_bpe$new())$
  train(normalizePath("small.txt"))$
  encode(c("hello world"))
#> [1]  57 427  93 275  61  53
```

