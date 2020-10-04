[![Build Status](https://github.com/google/vimscript-language-server/workflows/Rust/badge.svg)](https://github.com/google/vimscript-language-server/actions)

# Vimscript Language Server

Implementation of Language Server protocol for vimscript / vimL language in
Rust.

This project is still in very early development stage - it does not support all
of vimscript syntax and most features are not implemented yet.

The long term goal is to implement vimscript AST that will allow for:

* building language server
* building vimscript formatter, that vim plugins could use in CI
* building linter, that vim plugin could use in CI

The next steps:

* perform additional analysis on AST (e.g. variable tracking), to allow for features like renaming,
* build a foundation for formatter,
* build a foundation for generic linter (so that custom checks can be added),
* support all syntax of vimscript.

## Setup

### Build

```shell
$ cargo build
```

Then, copy built binary to a location that is in your PATH.

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
