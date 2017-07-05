#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations,
trivial_casts, trivial_numeric_casts, unsafe_code, unstable_features,
unused_import_braces, unused_qualifications)]

#![cfg_attr(feature="clippy", allow(unstable_features))]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy_pedantic))]
#![cfg_attr(feature="clippy", allow(non_ascii_literal))]

//! # encoji
//!
//! Encode a byte buffer into emojis, and decode an emoji string into a byte vector.
//!
//! Reimplementation of [`base_emoji`](https://github.com/pfrazee/base-emoji) including both
//! encoding and decoding.
//!
//! ## Example
//!
//! ```rust
//! let bytes = [0x6e, 0x6e, 0x6e, 0xcd];
//! let emojis = "🔥🔥🔥🚀";
//!
//! assert_eq!(encoji::to_string(&bytes), emojis);
//! assert_eq!(encoji::from_string(&emojis)[..], bytes);
//! ```
//!
//! ## Encoding (same as original implementation)
//!
//! Citing [the README](https://github.com/pfrazee/base-emoji/blob/master/README.md):
//!
//! > The emojis used are in `emojis.json`. There are 843 emojis there, but the
//! > converter reads sequences of 8 bits at a time, and so only maps the value to
//! > the first 256 of them. To stay consistent with other renderings, make sure you
//! > don't change the order of your emojis.json.
//!
//! ## Decoding
//!
//! String is split into graphemes by the
//! [`unicode-segmentation`](https://crates.io/crates/unicode-segmentation) crate in accordance
//! with the [Unicode Standard Annex #29](http://www.unicode.org/reports/tr29/). If the grapheme
//! isn't a mappable emoji it is skipped, else it is converted to a byte.
//!
//! ## License
//!
//! MIT. See included `LICENSE` file.
extern crate phf;
extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

include!(concat!(env!("OUT_DIR"), "/emojis.rs"));

/// Encode buffer as an encoji string
///
/// ```rust
/// let bytes = [0x6e, 0x6e, 0x6e, 0xcd];
/// let emojis = "🔥🔥🔥🚀";
///
/// assert_eq!(encoji::to_string(&bytes), emojis);
/// ```
pub fn to_string<T: AsRef<[u8]>>(buf: T) -> String {
    buf.as_ref()
        .iter()
        .flat_map(|c| EMOJIS.get(c))
        .map(|c| c.0)
        .collect::<String>()
}

/// Encode encoji string into buffer
///
/// ```rust
/// let emojis = "🔥🔥🔥🚀";
/// let bytes = vec![0x6e, 0x6e, 0x6e, 0xcd];
///
/// assert_eq!(encoji::from_string(&emojis), bytes);
/// ```
pub fn from_string(input: &str) -> Vec<u8> {
    UnicodeSegmentation::graphemes(input, true)
        .flat_map(|c| EMOJIS_REV.get(c))
        .map(|c| c.to_owned())
        .collect::<Vec<_>>()
}

/// Encode buffer as a string of emoji names
///
/// ```rust
/// let input = [0x6e, 0x6e, 0x6e, 0xcd];
/// let output = ":fire::fire::fire::rocket:";
/// assert_eq!(encoji::to_names(&input), output);
/// ```
pub fn to_names<T: AsRef<[u8]>>(buf: T) -> String {
    to_custom(buf, |_, name| format!(":{}:", name))
}

/// Encode buffer as custom-mapped string
///
/// ```rust
/// let input = [0xde];
/// let output = "<img src='/img/snowflake.png' alt='❄️' title='snowflake'>";
/// assert_eq!(
///     encoji::to_custom(&input, |ch, name| {
///         format!("<img src='/img/{}.png' alt='{}' title='{}'>",
///                 name, ch, name)
///     }),
///     output);
/// ```
pub fn to_custom<T: AsRef<[u8]>, F: Fn(&str, &str) -> String>(buf: T, f: F) -> String {
    buf.as_ref()
        .iter()
        .map(|c| {
            let emoji = EMOJIS.get(c).unwrap();
            f(emoji.0, emoji.1)
        })
        .collect::<Vec<_>>()
        .concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    static ALL_EMOJIS: &'static str = "💯🔢👍👎🎱🅰🚡✈️👽🚑⚓️👼💢😠🐜🍎⬇️⬅️➡️⬆️🎨🏧👶🍼🎈🎍🍌‼️📊💈🏀🛀🔋🐻🐝🍺🐞🔔🚴👙🎂🃏🌼📘🚙💙🐗⛵️💣📖📚💥\
                                       👢💐🎳👦🍞👰💼💔🐛💡🚌👤🌵🍰📆🐫🍬🚗🐈💿🏁🍒🐔🍫🎄👏🎬☁️🍸☕️💻🎊🚧🍪🌽👫🐄🐊👑🔮💘🌀💃🎯💨🌳💫🐕💵🐬🍩🚪💧👂🌍👓👊👣\
                                       🔥🔦💾🏈🍀🍤🍟🐸🎲💎👻🎁🌐⛳️🍇🍏🎸🔫🍔🔨👜🐣🌿🔆👠🐴⌛️🍨🎃🔑💋🐨🍃🔗🔒📢🔍📫🍁📣📝🎤🔬💰🐒🌙🐁🎥💪🍄🎹🎵👔📰🔕⛔️🚫\
                                       👃🔩🐙👌👐🐂🐼⛅️🐾🍑🍐🐧🎭☎️🐖🐽💊🍍🍕👇👈👉👆🚓🐩💩📯🍗🙏👛📌🐇🐎📻🐀🎀🍚💍🚀🐓🌹🚨📍🏃🎅📡🎷✂️🐚👕🚿💀😄🐌🐍❄️\
                                       ⛄️😭⚽️🔉👾💬⭐️🍓😎💦🏊💉🔭🎾💭🚽👅🎩🚥🏆🎺🐢🚦📼🎮🎻⌚️🐋🍷🐺🔧⚡️💤";

    #[test]
    fn all_bytes_to_emojis() {
        let all_code_points = (0..256)
            .collect::<Vec<_>>()
            .iter()
            .map(|cp| *cp as u8)
            .collect::<Vec<_>>();
        assert_eq!(to_string(&all_code_points[..]), ALL_EMOJIS);
    }

    #[test]
    fn all_emojis_to_bytes() {
        let all_code_points = (0..256)
            .collect::<Vec<_>>()
            .iter()
            .map(|cp| *cp as u8)
            .collect::<Vec<_>>();
        assert_eq!(from_string(&ALL_EMOJIS)[..], all_code_points[..]);
    }

    #[test]
    fn string_to_emojis() {
        let input = "encoji is the way of the future!";
        let output = "🐬🔥🐕🔦🌍👂🔋👂🍤🔋🍟💧🐬🔋💎🌳🎁🔋🔦🍩🔋🍟💧🐬🔋🍩🐸🍟🐸🍀🐬🐻";

        assert_eq!(to_string(input), output);
    }

    #[test]
    fn emojis_to_string() {
        let input = "🐬🔥🐕🔦🌍👂🔋👂🍤🔋🍟💧🐬🔋💎🌳🎁🔋🔦🍩🔋🍟💧🐬🔋🍩🐸🍟🐸🍀🐬🐻";
        let output = "encoji is the way of the future!";

        assert_eq!(String::from_utf8_lossy(&from_string(&input)[..]), output);
    }

    #[test]
    fn emojis_to_string_ingore_invalid_codepoints() {
        let input = "🐬🔥🐕🔦🌍👂🔋👂🍤🔋🍟💧🐬asdfasdf🔋💎🌳🎁🔋🔦🍩🔋🍟💧🐬🔋🍩🐸🍟🐸🍀🐬🐻\n";
        let output = "encoji is the way of the future!";

        assert_eq!(String::from_utf8_lossy(&from_string(&input)[..]), output);
    }
}
