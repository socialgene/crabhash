
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use seq_io::fasta::{Record, RefRecord};
use std::io::Write;
use std::path::PathBuf;
use glob::glob_with;
use glob::MatchOptions;
use sha2::{Sha512, Digest};
use base64;
//use grep_cli::stdout; //https://users.rust-lang.org/t/why-is-this-rust-loop-3x-slower-when-writing-to-disk/30489/3
use flate2::read::GzDecoder;
use scoped_threadpool::Pool;

fn get_reader(path: &PathBuf) -> Box<dyn BufRead + Send> {
    let mut filetype = "unzip";
    let filename_str = path.to_str().unwrap();
    let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => panic!("Error opening compressed file: {:?}.", error),
        };
    if filename_str.ends_with(".gz")  {filetype = "zip";}
    if filename_str.ends_with(".lz4") {filetype = "lz4";}
    let reader :Box<dyn BufRead + Send> = match filetype { 
        "zip" => Box::new(BufReader::new(GzDecoder::new(file))), 
        _ =>     Box::new(BufReader::new(file)), 
    }; 
    reader
}

fn parse_config(args: &[String]) -> &str {
    let dirpath = &args[1];
    dirpath
}

fn hash_and_write(record: &RefRecord, mut fasta_file: &File, mut tsv_file: &File) {
    let mut hasher = Sha512::new();
    let seq_str = String::from_utf8_lossy(record.seq()).to_string().replace('\n', ""); 
    hasher.update(&seq_str);
    let result = hasher.finalize();
    let ss = &result[0..24];
    let encoded = base64::encode_config(&ss, base64::URL_SAFE);
    fasta_file.write_all(b">").expect("write failed");
    fasta_file.write_all(&encoded.as_bytes()).expect("write failed");
    fasta_file.write_all(b"\n").expect("write failed");
    fasta_file.write_all(&seq_str.as_bytes()).expect("write failed");
    fasta_file.write_all(b"\n").expect("write failed");
    tsv_file.write_all(&encoded.as_bytes()).expect("write failed");
    tsv_file.write_all(b"\t").expect("write failed");
    tsv_file.write_all(record.id().unwrap().as_bytes()).expect("write failed");
    tsv_file.write_all(b"\n").expect("write failed");
}

fn create_outpaths(args: &[String], filename_string: &String) -> (File, File)  {
    let dirpath = &args[2];
    let fasta_pathbuf: PathBuf = [dirpath, &[&filename_string, ".fasta"].join("")].iter().collect();
    let tsv_pathbuf: PathBuf = [dirpath, &[&filename_string, ".tsv"].join("")].iter().collect();     
    let fasta_file = std::fs::File::create(&fasta_pathbuf).expect("create failed");
    let tsv_file = std::fs::File::create(&tsv_pathbuf).expect("create failed");

    return (fasta_file, tsv_file) ;
}

fn main()  {
    let args: Vec<String> = env::args().collect();
    let dirpath = parse_config(&args);

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    
    let max_workers = args[3].parse::<u32>().unwrap();
    let mut pool = Pool::new( max_workers );
    //let files: Vec<String> = Vec::from( &args[1..] );
        // Match the glob pattern, filtering out bad paths      
    pool.scoped( |scoped| {          
        for entry in glob_with(&dirpath, options).unwrap().filter_map(Result::ok) {
            // get outpath
            let filename = entry.file_name();
            let filename_string: String = filename.unwrap().to_str().unwrap().into();               
            let (fasta_file,tsv_file) = create_outpaths(&args, &filename_string);                       
            scoped.execute( move || {
                        // create output files
                        let buf = get_reader(&entry);
                        let mut reader = seq_io::fasta::Reader::new(buf);
                        while let Some(record) = reader.next() {
                            let record = record.unwrap();
                            hash_and_write(&record, &fasta_file, &tsv_file)                      
                        }
                    }
                );
            }
        }); 
}
