# My Leetcode Solution in Rust

[![Continuous integration](https://github.com/Aidan-Hamilton/leetcode/actions/workflows/ci.yaml/badge.svg?event=push)](https://github.com/Aidan-Hamilton/leetcode/actions/workflows/ci.yaml)

## Structure

## Usage

When running `cargo run`, it will direct to the CLI tool which currently accepts the following inputs:

- `$i` will initialize the template submission file of "question #id".
- `daily` will get the daily challenge.
- `random` will get a random problem.
- `solve $i` will move the submission file from the `problem` directory to the `solution` directory.
- `all` will create a file for all problems.

### Test Cases

Run `cargo test test_{id}` to test the solution for "question #id".
> ℹ️ **Note:** To see the output of your tests, you can run:
>
> ```sh
> cargo test test_{id} -- --show-output
> ```
>
> You can also run all tests by using `cargo test` or `cargo test -- --show-output`.

## Cloudflare

- To bypass Cloudflare, please `cp .env.sample .env`, and set `LEETCODE_COOKIE` with your cookie (can be found in Firefox or Chrome console).

## Repos

- [Rust-Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/intro.html)
- [Leetcode-Rust](https://github.com/aylei/leetcode-rust)
  - [Fork](https://github.com/tan-wei/leetcode-rust)
- [Test Cases](https://github.com/Aloxaf/leetcode_prelude)
