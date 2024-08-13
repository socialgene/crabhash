use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write, Read};
use std::path::Path;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use sha2::{Sha512, Digest};
use base64::Engine;
use seq_io::fasta::{self, Record};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_list_file> <output_fasta_file>", args[0]);
        std::process::exit(1);
    }

    let input_list_file = &args[1];
    let output_fasta_file = &args[2];

    let fasta_files = read_fasta_file_paths(input_list_file)?;
    let mut seen_hashes = HashSet::new();
    let output_file = File::create(output_fasta_file)?;
    let mut output = GzEncoder::new(output_file, Compression::default());

    let mut file_count = 0;
    let mut total_sequences = 0;

    for fasta_path in &fasta_files {
        file_count += 1;
        let file_path = Path::new(fasta_path);
        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(GzDecoder::new(file));
            match process_fasta_file(reader, &mut seen_hashes, &mut output) {
                Ok((sequences_in_file, ())) => total_sequences += sequences_in_file,
                Err(e) => eprintln!("Error processing file {}: {:?}", file_path.display(), e),
            }
            println!("Processed file {} ({}/{})", file_path.display(), file_count, fasta_files.len());
        } else {
            eprintln!("Could not open file: {}", file_path.display());
        }
    }

    println!("Total sequences processed: {}", total_sequences);

    Ok(())
}

fn read_fasta_file_paths(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn process_fasta_file<R: Read>(
    reader: R,
    seen_hashes: &mut HashSet<String>,
    output: &mut impl Write,
) -> Result<(usize, ()), io::Error> {
    let mut reader = fasta::Reader::new(reader);
    let mut sequence_count = 0;

    while let Some(record) = reader.next() {
        let record = record.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let seq_str = String::from_utf8_lossy(record.seq()).to_string().replace('\n', "").replace('\r', ""); 
        let mut hasher = Sha512::new();
        hasher.update(seq_str.as_bytes());
        let result = hasher.finalize();
        let ss = &result[0..24];
        let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(ss);

        if !seen_hashes.contains(&encoded) {
            seen_hashes.insert(encoded.clone());
            writeln!(output, ">{}", encoded)?;
            writeln!(output, "{}", seq_str)?;
            sequence_count += 1;
        }
    }

    Ok((sequence_count, ()))
}
