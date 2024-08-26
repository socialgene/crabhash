use std::collections::{HashSet, HashMap};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write, BufWriter, Read};
use std::path::Path;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use sha2::{Sha512, Digest};
use base64::Engine;
use seq_io::fasta::{self, Record};

const FLUSH_THRESHOLD: usize = 1000;  // Number of sequences to process before flushing

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} <input_list_file> <output_fasta_file> <source_files_tsv> <map_tsv>", args[0]);
        std::process::exit(1);
    }

    let input_list_file = &args[1];
    let output_fasta_file = &args[2];
    let source_files_tsv = &args[3];
    let map_tsv = &args[4];

    // Get log file name
    let log_file_name = get_log_file_name("crabhash.log");
    let log_file = OpenOptions::new().create(true).append(true).open(&log_file_name)?;
    let mut log = BufWriter::new(log_file);

    let fasta_files = read_fasta_file_paths(input_list_file)?;
    let mut seen_hashes = HashSet::new(); // Ensure uniqueness across all files
    let mut seen_sources = HashSet::new(); // Track written source file entries
    let output_file = File::create(output_fasta_file)?;
    let mut output = BufWriter::new(GzEncoder::new(output_file, Compression::default()));

    let mut file_count = 0;
    let mut total_sequences = 0;
    let mut total_added_to_nr = 0;
    let mut total_not_added_to_nr = 0;

    for (index, fasta_path) in fasta_files.iter().enumerate() {
        file_count += 1;
        let file_path = Path::new(fasta_path);
        let message = format!("Processing file {} ({}/{})", file_path.display(), file_count, fasta_files.len());
        println!("{}", message);
        writeln!(log, "{}", message)?;

        let mut source_files_map = HashMap::new();
        source_files_map.insert(index as u32, fasta_path.clone());

        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(GzDecoder::new(file));
            match process_fasta_file(reader, &mut seen_hashes, &mut seen_sources, &mut output, &mut log, index as u32, source_files_tsv, map_tsv, &source_files_map) {
                Ok((sequences_in_file, added_to_nr, not_added_to_nr)) => {
                    total_sequences += sequences_in_file;
                    total_added_to_nr += added_to_nr;
                    total_not_added_to_nr += not_added_to_nr;
                    let message = format!("File {}: {} sequences processed, {} added to nr, {} not added to nr",
                                          file_path.display(), sequences_in_file, added_to_nr, not_added_to_nr);
                    println!("{}", message);
                    writeln!(log, "{}", message)?;
                },
                Err(e) => {
                    let message = format!("Error processing file {}: {:?}", file_path.display(), e);
                    eprintln!("{}", message);
                    writeln!(log, "{}", message)?;
                },
            }
        } else {
            let message = format!("Could not open file: {}", file_path.display());
            eprintln!("{}", message);
            writeln!(log, "{}", message)?;
        }
    }

    let summary = format!("Total sequences processed: {}", total_sequences);
    println!("{}", summary);
    writeln!(log, "{}", summary)?;

    let summary = format!("Total sequences added to nr: {}", total_added_to_nr);
    println!("{}", summary);
    writeln!(log, "{}", summary)?;

    let summary = format!("Total sequences not added to nr: {}", total_not_added_to_nr);
    println!("{}", summary);
    writeln!(log, "{}", summary)?;

    log.flush()?;  // Ensure all log data is written to disk
    Ok(())
}

fn get_log_file_name(base_name: &str) -> String {
    let mut log_file_name = String::from(base_name);
    let mut count = 1;
    while Path::new(&log_file_name).exists() {
        log_file_name = format!("crabhash_{}.log", count);
        count += 1;
    }
    log_file_name
}

fn read_fasta_file_paths(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn process_fasta_file<R: Read>(
    reader: R,
    seen_hashes: &mut HashSet<[u8; 24]>,
    seen_sources: &mut HashSet<String>,
    output: &mut impl Write,
    log: &mut BufWriter<File>,
    file_index: u32,
    source_files_tsv: &str,
    map_tsv: &str,
    source_files_map: &HashMap<u32, String>,
) -> Result<(usize, usize, usize), io::Error> {
    let mut reader = fasta::Reader::new(reader);
    let mut sequence_count = 0;
    let mut added_to_nr = 0;
    let mut not_added_to_nr = 0;
    let mut hasher = Sha512::new();
    let mut protein_map = Vec::new();
    let mut unique_map_entries = HashSet::new();

    while let Some(record) = reader.next() {
        let record = record.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let mut seq_bytes = Vec::new();
        for line in record.seq_lines() {
            seq_bytes.extend_from_slice(line);
        }

        while seq_bytes.first().map_or(false, |&c| c == b'*' || c.is_ascii_whitespace()) {
            seq_bytes.remove(0);
        }
        while seq_bytes.last().map_or(false, |&c| c == b'*' || c.is_ascii_whitespace()) {
            seq_bytes.pop();
        }

        seq_bytes.make_ascii_uppercase();

        hasher.update(&seq_bytes);
        let result = hasher.finalize_reset(); // reset the hasher for the next sequence
        let ss = &result[0..24];
        let encoded: [u8; 24] = ss.try_into().expect("Slice with incorrect length");

        if let Ok(id) = record.id() {
            let map_entry = (file_index, encoded, id.to_string());
            if unique_map_entries.insert(map_entry.clone()) {
                protein_map.push(map_entry);
            }
        }

        if !seen_hashes.contains(&encoded) {
            seen_hashes.insert(encoded.clone());
            writeln!(output, ">{}", base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(ss))?;
            writeln!(output, "{}", String::from_utf8_lossy(&seq_bytes))?;
            added_to_nr += 1;
        } else {
            not_added_to_nr += 1;
        }

        sequence_count += 1;

        if sequence_count % FLUSH_THRESHOLD == 0 {
            output.flush()?;
            log.flush()?;  // Flush the log file periodically
            append_source_files_tsv(source_files_tsv, source_files_map, seen_sources)?;
            append_map_tsv(map_tsv, &protein_map)?;
            protein_map.clear();
        }
    }

    // Final flush after processing the entire file
    output.flush()?;
    log.flush()?;  // Final log flush after file processing
    append_source_files_tsv(source_files_tsv, source_files_map, seen_sources)?;
    append_map_tsv(map_tsv, &protein_map)?;

    Ok((sequence_count, added_to_nr, not_added_to_nr))
}

fn append_source_files_tsv(file_path: &str, data: &HashMap<u32, String>, seen_sources: &mut HashSet<String>) -> io::Result<()> {
    let file = OpenOptions::new().create(true).append(true).open(file_path)?;
    let gz_file = GzEncoder::new(file, Compression::default());
    let mut writer = BufWriter::new(gz_file);
    
    for (index, path) in data {
        // Only write the entry if it's not already in seen_sources
        if seen_sources.insert(path.clone()) {
            writeln!(writer, "{}\t{}", index, path)?;
        }
    }

    writer.flush()?;  // Ensure data is written to the file
    Ok(())
}

fn append_map_tsv(file_path: &str, data: &[(u32, [u8; 24], String)]) -> io::Result<()> {
    let file = OpenOptions::new().create(true).append(true).open(file_path)?;
    let gz_file = GzEncoder::new(file, Compression::default());
    let mut writer = BufWriter::new(gz_file);
    for (file_index, hash, original_id) in data {
        writeln!(writer, "{}\t{}\t{}", file_index, base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash), original_id)?;
    }
    Ok(())
}
