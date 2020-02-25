# Vimscript Language Server

Implementation of Language Server protocol for vimscript / vimL language in Rust.

This project is under active development.

## Setup

### Build

```shell
$ cargo build
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
