# chunked-transfer-cli

Command-line tool to **encode into or decode from [HTTP chunked transfer coding](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Transfer-Encoding)** ([RFC 7230, section 4.1](https://www.rfc-editor.org/rfc/rfc7230#section-4.1)).

[![crates.io version](https://img.shields.io/crates/v/chunked_transfer_cli.svg)](https://crates.io/crates/chunked_transfer_cli)
![ISC-licensed](https://img.shields.io/github/license/derhuerst/chunked-transfer-cli.svg)
[![support me via GitHub Sponsors](https://img.shields.io/badge/support%20me-donate-fa7664.svg)](https://github.com/sponsors/derhuerst)
[![chat with me on Twitter](https://img.shields.io/badge/chat%20with%20me-on%20Twitter-fa7664.svg)](https://twitter.com/derhuerst)

I'm a Rust beginner, so the code might be unelegant. 🙈

## Installing

```shell
cargo install chunked_transfer_cli
```


## Usage

```txt
chunked_transfer_cli 0.1.0
Command-line tool to encode into or decode from HTTP chunked transfer coding (RFC 7230).

USAGE:
    chunked_transfer_cli [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    decode    decodes chunked data
    encode    encodes data as chunked
    help      Print this message or the help of the given subcommand(s)
```

```txt
decodes chunked data

USAGE:
    chunked_transfer_cli decode

OPTIONS:
    -h, --help    Print help information
```

```txt
encodes data as chunked

USAGE:
    chunked_transfer_cli encode [OPTIONS]

OPTIONS:
        --chunk-size <CHUNK_SIZE>    number of bytes per chunk [default: 1024]
    -h, --help                       Print help information
```