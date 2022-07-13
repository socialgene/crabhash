
use glob::glob_with;
use glob::MatchOptions;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use seq_io::fasta::{Record, RefRecord};
use std::io::Write;
use std::path::PathBuf;
use sha2::{Sha512, Digest};
use base64;
use flate2::read::GzDecoder;
use scoped_threadpool::Pool;
use clap::Parser;



//***********************************************************************************
#[derive(Parser, Debug)]
#[clap(author, version, about = None, long_about = None)]
struct Args {
  
  #[clap(short, long, value_parser)]
  glob: String,
  
  #[clap(short, long, value_parser)]
  outdir: String,
  
  #[clap(short, long, value_parser, default_value_t = 1)]
  nsplits: u8,
  
  #[clap(short, long, value_parser, default_value_t = 1)]
  ncpus: u8,

}

//***********************************************************************************
<- 
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

fn hash(seq_str:  &String) -> String{
    let mut hasher = Sha512::new();
    hasher.update(seq_str);
    let result = hasher.finalize();
    let ss = &result[0..24];
    let mm = base64::encode_config(&ss, base64::URL_SAFE);
    mm
}


fn hash_and_write(record: &RefRecord, mut fasta_file: &File, mut tsv_file: &File) {
    let seq_str = String::from_utf8_lossy(record.seq()).to_string().replace('\n', "").replace('\r', ""); 
    let encoded = hash(&seq_str);

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

fn main() {
    let args = Args::parse();

    for _ in 0..args.ncpus {
        let bro = args.ncpus + 1;
        println!("Hello {}!", bro.to_string())
    }

        

        let arg_outdir = matches.value_of("outdir").unwrap();
        let arg_nsplits = matches.value_of("nsplits").unwrap();
        let arg_ncpus = matches.value_of("ncpus").unwrap();

        let options = MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };
    // The number of splits will be equal to the number of 
    let mut pool = Pool::new( &arg_glob );
    //let files: Vec<String> = Vec::from( &args[1..] );
        // Match the glob pattern, filtering out bad paths      
    pool.scoped( |scoped| {      
        let filepaths = glob_with(&arg_outdir, options).unwrap().filter_map(Result::ok) ;
        let mut current_count: usize = 0;
        let mut file_int: usize = 0;

        let fc = &filepaths.count();        
        let input_per_output = 3;
        
        for entry in glob_with(&arg_outdir, options).unwrap().filter_map(Result::ok){
            // get outpath
            if current_count >= input_per_output {
                println!("{}", "hiya");
                file_int += 1;
                current_count = 0;
            }
            println!("{}", &current_count.to_string());
            current_count += 1;
            let filename_string: String = file_int.to_string();               
            let (fasta_file,tsv_file) = create_outpaths(&args, &filename_string);       
            println!("{}", &current_count.to_string());
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

