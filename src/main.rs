use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    input: String,
    output: String,
    #[arg(short, long, value_enum, default_value_t = Key::Cowabunga)]
    key: Key,
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
	BlizzardArcade  = 0x93C8C18A
}

fn main() {
    let cli: Cli = Cli::parse();
    let file = File::open(&cli.input).unwrap();
    let mut file_reader = BufReader::new(file);

    let file_size = fs::metadata(&cli.input).unwrap().len();

    let output_file = File::create(&cli.output).unwrap();
    let mut output_file_writer = BufWriter::new(output_file);

    let mut buffer = [0; 0x10000];

    let file_size_padding = file_size % 0x10000;
    let file_size = file_size - file_size_padding;

    for index in (0..file_size).step_by(0x10000) {
        file_reader.read_exact(&mut buffer).unwrap();
        let mut vec = buffer.to_vec();
        decrypt_block(&mut vec, index as u32, cli.key);
        output_file_writer.write_all(vec.as_slice()).unwrap();
    }

    let mut vec = Vec::new();
    for _ in 0..file_size_padding {
        let mut buffer = [0u8];
        file_reader.read_exact(&mut buffer).unwrap();
        vec.push(buffer[0]);
    }
    decrypt_block(&mut vec, file_size as u32, cli.key);
    output_file_writer.write_all(vec.as_slice()).unwrap();
}

fn decrypt_block(block: &mut Vec<u8>, offset_in_file: u32, game_key: Key) {
    let mut sum = offset_in_file.wrapping_mul(0xCC9E2D51);
    let mut key = get_key( (offset_in_file & 0xFFFFFFFC).wrapping_mul(0xCC9E2D51), game_key);
    for index in 0..block.len() {
        let iter = (offset_in_file.wrapping_add( index as u32) & 3) << 3;
        block[index] ^= (key >> iter) as u8;
        sum = sum.wrapping_sub(0x3361D2AF);
        if iter == 24 {
            key = get_key(sum, game_key);
        }
    }
}

fn get_key(sum: u32, game_key: Key) -> u32 {
    let temp1 = sum.rotate_left(15).wrapping_mul(0x1B873593) ^ (game_key as u32);
    let temp2 = temp1.rotate_left(13).wrapping_mul(5).wrapping_sub(0x19AB949C);
    let temp3 = (temp2 ^ 0x40000) >> 16;
    let temp4 = (temp2 ^ temp3).wrapping_mul(0x85EBCA6B);
    let temp5 = (temp4 ^ (temp4 >> 13)).wrapping_mul(0xC2B2AE35);
    temp5 ^ (temp5 >> 16)
}