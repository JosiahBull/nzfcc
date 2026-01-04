# NZFCC - New Zealand Financial Category Codes

A Rust library providing types and utilities for working with the New Zealand Financial Category Codes (NZFCC) system.

The NZFCC is a standardized categorization system for financial transactions in New Zealand. For more information, visit [https://nzfcc.org/](https://nzfcc.org/).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nzfcc = "0.1"
```

## Regenerating Code

To regenerate the category definitions from the latest NZFCC data:

```bash
cargo run -p nzfcc-generator
```

This will:
1. Download the latest categories from `https://nzfcc.org/downloads/categories.json`
2. Save a formatted copy to `categories.json` in the repository root
3. Generate the Rust code in `crates/nzfcc/src/generated/`

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
