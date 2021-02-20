# How to Contribute

We'd love to accept your patches and contributions to this project. There are
just a few small guidelines you need to follow.

## Contributor License Agreement

Contributions to this project must be accompanied by a Contributor License
Agreement. You (or your employer) retain the copyright to your contribution;
this simply gives us permission to use and redistribute your contributions as
part of the project. Head over to <https://cla.developers.google.com/> to see
your current agreements on file or to sign a new one.

You generally only need to submit a CLA once, so if you've already submitted one
(even if it was for a different project), you probably don't need to do it
again.

## Code reviews

All submissions, including submissions by project members, require review. We
use GitHub pull requests for this purpose. Consult
[GitHub Help](https://help.github.com/articles/about-pull-requests/) for more
information on using pull requests.

## Community Guidelines

This project follows [Google's Open Source Community
Guidelines](https://opensource.google/conduct/).

## Building

### Install rust

Follow instructions on https://www.rust-lang.org/tools/install to install rust.

### Setup a fork

1. Create a fork (click Fork button in github)

2. Download fork

```shell
$ git clone git@github.com:YOUR_USERNAME/vimscript-language-server.git
```

3. Specify a new remote `upstream`

```shell
$ git remote add upstream https://github.com/google/vimscript-language-server.git
```

To sync a fork, run:

```shell
$ git checkout master
$ git fetch upstream master
$ git rebase upstream/master
```

To sync your feature branch, run:

```shell
$ git checkout my_feature
$ git rebase master
```

### Verify everything works

```shell
$ cargo test
```

### Update expect file

To automatically update the expected file, run:

```shell
$ UPDATE_EXPECT=1 cargo test
```

### Cargo watch (optional)

```shell
$ cargo install cargo-watch
```

This crate allows running tests (or build) in watch mode (automatically reruns
the tests when some file is changed). To start tests in watch mode, run:

```shell
$ cargo watch -x test
```

## Architecture

This codebase is very strongly inspired by rust-analyzer - it is strongly
recommended to read
https://github.com/rust-analyzer/rust-analyzer/blob/cf5bdf464cad7ceb9a67e07985a3f4d3799ec0b6/docs/dev/guide.md#guide-to-rust-analyzer.
