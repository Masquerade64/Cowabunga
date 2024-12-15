use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use clap::{Parser, ValueEnum};
use std::process;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    input: String,
    output: String,
    #[arg(short, long, value_enum, default_value_t = Key::Cowabunga)]
    key: Key,
    #[arg(short = 'c', long = "custom", value_name = "CUSTOM_KEY")]
    custom_key: Option<String>,
}

#[derive(Clone, ValueEnum, Copy)]
#[repr(u32)]
enum Key {
    Cowabunga       = 0x3F04B286,
    Atari           = 0x2EEA4C8B,
    AtariDLC1       = 0x07853295,
    AtariDLC2       = 0x3AA19D18,
    MakingKarateka  = 0x920DEA25,
    GarbagePailKids = 0xAA31713C,
    JeffMinter      = 0x34A4C18E,
    BlizzardArcade  = 0x93C8C18A,
    MightyMorphin   = 0xFA5E893B
}

fn main() {
    let cli: Cli = Cli::parse();

    // If a custom key is provided, use it. Otherwise, use the default or chosen enum key.
    let key = match cli.custom_key {
        Some(key_str) => {
            // Remove the '0x' prefix if present and parse the custom key from a hexadecimal string to u32
            let key_str = key_str.trim_start_matches("0x");  // Remove the "0x" prefix
            match u32::from_str_radix(&key_str, 16) {
                Ok(parsed_key) => parsed_key,
                Err(_) => {
                    eprintln!("Error: Invalid custom key '{}'. Please provide a valid 32-bit hexadecimal value.", key_str);
                    process::exit(1);  // Exit with error code 1
                }
            }
        }
        None => cli.key as u32, // Use the predefined key from the enum if no custom key is provided
    };

    let file = File::open(&cli.input).unwrap();
    let mut file_reader = BufReader::new(file);

    let file_size = fs::metadata(&cli.input).unwrap().len();

    // Create a temporary buffer to hold decrypted data
    let mut buffer = [0; 0x10000];

    let file_size_padding = file_size % 0x10000;
    let file_size = file_size - file_size_padding;

    let mut output_file_writer: Option<BufWriter<File>> = None;

    for index in (0..file_size).step_by(0x10000) {
        file_reader.read_exact(&mut buffer).unwrap();
        let mut vec = buffer.to_vec();
        decrypt_block(&mut vec, index as u32, key);

        // If we haven't initialized the output writer yet, do it now
        if output_file_writer.is_none() {
            let output_file = File::create(&cli.output).unwrap();
            output_file_writer = Some(BufWriter::new(output_file));
        }

        output_file_writer.as_mut().unwrap().write_all(vec.as_slice()).unwrap();
    }

    let mut vec = Vec::new();
    for _ in 0..file_size_padding {
        let mut buffer = [0u8];
        file_reader.read_exact(&mut buffer).unwrap();
        vec.push(buffer[0]);
    }
    decrypt_block(&mut vec, file_size as u32, key);

    // If we haven't initialized the output writer yet, do it now
    if output_file_writer.is_none() {
        let output_file = File::create(&cli.output).unwrap();
        output_file_writer = Some(BufWriter::new(output_file));
    }

    output_file_writer.as_mut().unwrap().write_all(vec.as_slice()).unwrap();

    // After conversion, check if the output file is a valid ZIP
    if !is_valid_zip(&cli.output) {
        // Clean up the output file if it's not a valid ZIP
        fs::remove_file(&cli.output).unwrap();
        eprintln!("Error: The output file '{}' is not a valid ZIP file.", cli.output);
        process::exit(1);  // Exit with error code 1
    }

    println!("The output file '{}' was successfully converted and is a valid ZIP file.", cli.output);
}

/// Checks if a file is a valid ZIP by looking for the "PK" header.
fn is_valid_zip(file_path: &str) -> bool {
    if let Ok(mut file) = File::open(file_path) {
        let mut header = [0; 2];
        if let Ok(_) = file.read_exact(&mut header) {
            return &header == b"PK";  // Checks if the first two bytes are "PK"
        }
    }
    false
}

fn decrypt_block(block: &mut Vec<u8>, offset_in_file: u32, game_key: u32) {
    let mut sum = offset_in_file.wrapping_mul(0xCC9E2D51);
    let mut key = get_key((offset_in_file & 0xFFFFFFFC).wrapping_mul(0xCC9E2D51), game_key);
    for index in 0..block.len() {
        let iter = (offset_in_file.wrapping_add(index as u32) & 3) << 3;
        block[index] ^= (key >> iter) as u8;
        sum = sum.wrapping_sub(0x3361D2AF);
        if iter == 24 {
            key = get_key(sum, game_key);
        }
    }
}

fn get_key(sum: u32, game_key: u32) -> u32 {
    let temp1 = sum.rotate_left(15).wrapping_mul(0x1B873593) ^ game_key;
    let temp2 = temp1.rotate_left(13).wrapping_mul(5).wrapping_sub(0x19AB949C);
    let temp3 = (temp2 ^ 0x40000) >> 16;
    let temp4 = (temp2 ^ temp3).wrapping_mul(0x85EBCA6B);
    let temp5 = (temp4 ^ (temp4 >> 13)).wrapping_mul(0xC2B2AE35);
    temp5 ^ (temp5 >> 16)
}
