
use std::env;
use std::fs::File;
use seq_io::fasta::{Record, RefRecord};
use std::io::{BufWriter, Write};
use std::fs;

use std::path::PathBuf;
use std::path::Path;
use glob::glob_with;
use sha2::{Sha512, Digest};
use base64;
//use grep_cli::stdout; //https://users.rust-lang.org/t/why-is-this-rust-loop-3x-slower-when-writing-to-disk/30489/3
use scoped_threadpool::Pool;


mod filehandling;

use base64::{Engine as _, engine::general_purpose};





fn main()  {
    let args: Vec<String> = env::args().collect();

    let input_file = Path::new(&args[1]);
    
    let filename = &input_file.file_stem().unwrap();
    let filename_string = &input_file.file_stem().unwrap().to_str().unwrap();

    let outdir = Path::new(&args[2]);

    let mut tsv_pathbuf = PathBuf::from(&outdir);

    tsv_pathbuf.push(&[&filename_string, ".tsv"].join(""));
    
    let (mut input, _) = filehandling::get_input(&args[1]).unwrap();

    let mut reader = seq_io::fasta::Reader::new(&mut input);

    let mut n = 0;
    print!("Finished 0 million sequences");
    let tt = std::fs::File::create(&tsv_pathbuf).expect("create failed");
    let mut tsv_file = BufWriter::new(tt);


    while let Some(record) = reader.next() {
        let record2 = record.unwrap();
        let mut hasher = Sha512::new();
        let seq_str = String::from_utf8_lossy(record2.seq()).to_string().replace('\n', "").replace('\r', ""); 
        hasher.update(&seq_str);
        let result = hasher.finalize();
        let ss = &result[0..24];
        // Conistent with Python's RFC 3548 base64.urlsafe_b64encode
        let encoded = general_purpose::URL_SAFE_NO_PAD.encode(&ss);

        let mut s = String::new();

        s.push_str(&encoded);
        s.push_str("\t");
        s.push_str(record2.id().unwrap());
        s.push_str("\n");
        
        if n % 1000000 == 0 {
            print!("\r Finished {} x million sequences", n/1000000);
        }
        n += 1;
        tsv_file.write_all(&s.as_bytes()).expect("write failed");


    }   


    
    tsv_file.flush().unwrap();
}
