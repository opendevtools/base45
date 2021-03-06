# Base45

[![Test and publish Base45](https://github.com/opendevtools/base45/workflows/Release/badge.svg?branch=main)](https://github.com/opendevtools/base45/actions?query=workflow%3A%22Release%22)
[![Crate](https://img.shields.io/crates/v/base45.svg)](https://crates.io/crates/base45)
[![API](https://docs.rs/base45/badge.svg)](https://docs.rs/base45)

A encoder/decoder for base45 that is fully compatible with
[draft-faltstrom-base45-02](https://www.ietf.org/id/draft-faltstrom-base45-02.txt). When encoding QR or Aztec codes a different scheme than the standard base64, base32, and base16 is needed.

## Installation

```toml
[dependencies]
base45 = "1.0.1"
```

## Benchmarks

Benchmarks were created using [Criterion](https://github.com/bheisler/criterion.rs). The benchmarks test encoding and decoding of the string "The quick brown fox jumps over the lazy dog". Test computer is a MacBook 16" (2019) with 2,6 GHz 6-Core Intel Core i7 and 32 GB 2667 MHz DDR4.

```
encode long string                time:   [6.0716 us 6.1040 us 6.1377 us]
encode long string from buffer    time:   [6.0220 us 6.0547 us 6.0885 us]
decode long string                time:   [1.0876 us 1.0993 us 1.1105 us]
```
