# Base45

[![Test and publish Base45](https://github.com/opendevtools/base45/workflows/Release/badge.svg?branch=main)](https://github.com/opendevtools/base45/actions?query=workflow%3A%22Release%22)

A encoder/decoder for base45 that is fully compatible with
[draft-faltstrom-base45-02.txt](https://www.ietf.org/id/draft-faltstrom-base45-02.txt).

## Usage

### Encode

```rust
use base45;

let encoded = base45::encode("Hello!!")
// %69 VD92EX0
```

### Decode

```rust
use base45;

let decoded = base45::decode("%69 VD92EX0").unwrap()
// Hello!!
```

## Benchmarks

Benchmarks were created using [Criterion](https://github.com/bheisler/criterion.rs). The benchmarks test encoding and decoding of the string "The quick brown fox jumps over the lazy dog". Test computer is a MacBook 16" (2019) with 2,6 GHz 6-Core Intel Core i7 and 32 GB 2667 MHz DDR4.

```
Benchmarking encode long string: Collecting 100 samples in estimated 5.
encode long string      time:   [17.590 us 17.817 us 18.054 us]
                        change: [-92.772% -92.631% -92.467%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

Benchmarking decode long string: Collecting 100 samples in estimated 5.
decode long string      time:   [3.2382 us 3.2938 us 3.3505 us]
                        change: [-98.402% -98.364% -98.322%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
```
