[![Build Status](https://github.com/google/vimscript-language-server/workflows/Rust/badge.svg)](https://github.com/google/vimscript-language-server/actions)

# Vim script Language Server

Implementation of Language Server protocol for Vim script / vimL language in
Rust.

This project is still in very early development stage - it does not support all
of Vim script syntax and most features are not implemented yet.

The long term goal is to implement Vim script AST that will allow for:

* building language server
* building Vim script formatter, that vim plugins could use in CI
* building linter, that vim plugin could use in CI

The next steps:

* perform additional analysis on AST (e.g. variable tracking), to allow for features like renaming,
* build a foundation for formatter,
* build a foundation for generic linter (so that custom checks can be added),
* support all syntax of Vim script.


## Setup

### Build

```shell
$ cargo build --release
```

Then, copy built binary to a location that is in your PATH.

Alternatively, just run:

```shell
$ cargo install --path .
```

### Configure in vim-lsp

```vim
if executable('vimscript-language-server')
  au User lsp_setup call lsp#register_server({
          \ 'name': 'vimscript-language-server',
          \ 'cmd': {server_info->WrapLspTee(['vimscript-language-server'])},
          \ 'whitelist': ['vim'],
          \ })
endif
```

## Contributing

See [Contributing.md](CONTRIBUTING.md).

--------------------------------------------------------------------------------

This is not an officially supported Google product
