#![feature(io_read_to_string)]

use clap::{Parser, Subcommand};
use chunked_transfer::{Decoder, Encoder};
use std::io::{Result, stdin, stdout, copy};

const DEFAULT_CHUNK_SIZE: usize = 1024;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Cli {
	// /// decode chunked transfer-coded data, instead of encoding
	// #[clap(short, long, value_parser)]
	// decode: bool,

	#[clap(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
	/// decodes chunked data
	Decode {
	},
	/// encodes data as chunked
	Encode {
		/// number of bytes per chunk
		#[clap(long, value_parser, default_value_t = DEFAULT_CHUNK_SIZE)]
		chunk_size: usize,
	},
}

fn decode() -> Result<()> {
	let mut decoder = Decoder::new(stdin());
	copy(&mut decoder, &mut stdout())?;

	Ok(())
}

fn encode(chunk_size: usize) -> Result<()> {
	let mut encoder = Encoder::with_chunks_size(stdout(), chunk_size);
	copy(&mut stdin(), &mut encoder)?;

	Ok(())
}

fn main() {
	let cli = Cli::parse();

	let result = match &cli.command {
		Some(Commands::Decode {}) => decode(),
		Some(Commands::Encode { chunk_size }) => encode(chunk_size.clone()),
		None => encode(DEFAULT_CHUNK_SIZE),
	};

	if let Err(err) = result {
		eprintln!("Error: {:?}", err);
		std::process::exit(1);
	}
}
