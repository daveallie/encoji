[![Build Status](https://travis-ci.org/daveallie/encoji.svg?branch=master)](https://travis-ci.org/daveallie/encoji)
[![Crate](https://img.shields.io/crates/v/encoji.svg)](https://crates.io/crates/encoji)
[![Docs](https://docs.rs/encoji/badge.svg)](https://docs.rs/encoji)
![License](https://img.shields.io/crates/l/encoji.svg)
[![Downloads](https://img.shields.io/crates/d/encoji.svg)](https://crates.io/crates/encoji)

# encoji

Encode a byte buffer into emojis, and decode an emoji string into a byte vector.

Reimplementation of [`base_emoji`](https://github.com/pfrazee/base-emoji) including both
encoding and decoding.

### Example

```rust
let bytes = [0x6e, 0x6e, 0x6e, 0xcd];
let emojis = "🔥🔥🔥🚀";

assert_eq!(encoji::to_string(&bytes), emojis);
assert_eq!(encoji::from_string(&emojis)[..], bytes);
```

### Encoding (same as original implementation)

Citing [the README](https://github.com/pfrazee/base-emoji/blob/master/README.md):

> The emojis used are in `emojis.json`. There are 843 emojis there, but the
> converter reads sequences of 8 bits at a time, and so only maps the value to
> the first 256 of them. To stay consistent with other renderings, make sure you
> don't change the order of your emojis.json.

### Decoding

String is split into graphemes by the
[`unicode-segmentation`](https://crates.io/crates/unicode-segmentation) crate in accordance
with the [Unicode Standard Annex #29](http://www.unicode.org/reports/tr29/). If the grapheme
isn't a mappable emoji it is skipped, else it is converted to a byte.

### License

MIT. See included `LICENSE` file.
