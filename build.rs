#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write, Read};
use std::path::Path;

#[derive(Deserialize)]
struct Emoji {
    #[serde(rename(deserialize = "char"))]
    character: String,
    name: String,
}

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("emojis.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut emoji_file = File::open("emojis.json").expect("Can't open emoji file");
    let mut buffer = String::new();
    emoji_file.read_to_string(&mut buffer).expect(
        "Can't read emoji file",
    );

    let emojis: Vec<Emoji> = serde_json::from_str(&buffer).expect("Invalid JSON im emoji file");
    assert!(emojis.len() == 256);

    generate_standard_map(&mut file, &emojis);
    write!(&mut file, "\n").unwrap();
    generate_reverse_map(&mut file, &emojis);
}

fn generate_standard_map(file: &mut BufWriter<File>, emojis: &Vec<Emoji>) {
    write!(
        file,
        "/// Compile time generated lookup table for Emojis.\n"
    ).unwrap();
    write!(file, "/// \n").unwrap();
    write!(
        file,
        "static EMOJIS: phf::Map<u8, (&'static str, &'static str)> = "
    ).unwrap();

    let mut m = phf_codegen::Map::new();
    for (idx, emoji) in emojis.iter().enumerate().take(256) {
        m.entry(
            idx as u8,
            &format!("({:?}, {:?})", emoji.character, emoji.name),
        );
    }
    m.build(file).unwrap();
    write!(file, ";\n").unwrap();
}

fn generate_reverse_map(file: &mut BufWriter<File>, emojis: &Vec<Emoji>) {
    write!(
        file,
        "/// Compile time generated reverse lookup table for Emojis.\n"
    ).unwrap();
    write!(file, "/// \n").unwrap();
    write!(file, "static EMOJIS_REV: phf::Map<&'static str, u8> = ").unwrap();

    let mut m = phf_codegen::Map::new();
    for (idx, emoji) in emojis.iter().enumerate().take(256) {
        m.entry(emoji.character.clone(), &format!("{}", idx as u8));
    }
    m.build(file).unwrap();
    write!(file, ";\n").unwrap();
}
