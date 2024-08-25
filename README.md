### Background 

The hash method (*sha512t24u*) used is described here:
https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7714221/


Due to personal time constraints much of the code was re-edited by GPT-4o for different functionality than the orginal.

### Use
```sh
git clone https://github.com/socialgene/crabhash.git

cd crabhash

cargo build --release 

./target/release/crabhash <input_list_file> <output_fasta_file> <source_files_tsv> <map_tsv> 

```

### Inputs

**<input_list_file>**
```
some/file/path/input_file_1.faa.gz
some/file/path/input_file_2.faa.gz
``` 

### Outputs

(amino acid sequences below are truncated)

**<output_fasta_file>.gz** (non-redundant fasta file)
```
>bSWOkfS_463jA18t6HczuoLGzPmXrZIG
MKTA
>VTuZDzLbzHsOlS8Lg7IhBRzE-8xrk2zg
XVFG
```

**<source_files_tsv>.gz** (file_id, file_path)
```
0       some/file/path/input_file_1.faa.gz
1       some/file/path/input_file_2.faa.gz
```

**<map_tsv>.gz** (file_id, protein_hash, defline_identifier)
```
0       bSWOkfS_463jA18t6HczuoLGzPmXrZIG        defline_identifier
1       bSWOkfS_463jA18t6HczuoLGzPmXrZIG        defline_identifier
```

