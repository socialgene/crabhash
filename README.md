### Background 

The hash method (*sha512t24u*) used is described here:
https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7714221/


### Use
```sh
git clone https://github.com/socialgene/crabhash.git

cd crabhash

cargo build --release 

./target/release/hash_proteins \
    '/glob/pattern/to/match/fasta/files' \
    '/output/directory/' \
    ncpus
```

### Outputs

Takes:

**input_file.faa.gz**
```
>WP_188403107.1 hypothetical protein [Fictibacillus barbaricus]
MEQKETFFVTAKGDIQPLPTDDHVHYFEIQATYDEKQQIDHLFTQIHANNKQEGLDIFSPKRHFSESHAEYHRGKDSKLV
YELFRYIYLLGTEKTKREIEEMNVLPELFESSHTTIEKVK
>WP_188403108.1 hypothetical protein [Fictibacillus barbaricus]
MKLPVHNQPFIKADPFVLEKTEVISAVKDYLKREGYETEILEDIYGVNLAAENEYHTLLIAAQGNTSELQLLSHKYPATQ
NETNFDKLIVDLLKHHEKNPAKTLVLASPDTPLFRDKAENIKQALDDLGIVRFWVKENGSIEWE
```

And outputs:

**input_file.faa.gz.fasta**
```
>POt5XgLIthEqNIqvXB1PiFGkhjgwufcK
MEQKETFFVTAKGDIQPLPTDDHVHYFEIQATYDEKQQIDHLFTQIHANNKQEGLDIFSPKRHFSESHAEYHRGKDSKLV
YELFRYIYLLGTEKTKREIEEMNVLPELFESSHTTIEKVK
>KFMMpfPfblX3Bbrhtk40mJ5rwFi40OfA
MKLPVHNQPFIKADPFVLEKTEVISAVKDYLKREGYETEILEDIYGVNLAAENEYHTLLIAAQGNTSELQLLSHKYPATQ
NETNFDKLIVDLLKHHEKNPAKTLVLASPDTPLFRDKAENIKQALDDLGIVRFWVKENGSIEWE
```

***input_file.faa.gz.tsv**
```
POt5XgLIthEqNIqvXB1PiFGkhjgwufcK	WP_188403107.1
KFMMpfPfblX3Bbrhtk40mJ5rwFi40OfA	WP_188403108.1
```



_____________

```
git clone https://github.com/nickmachnik/fastatools.git
cd fastatools
cargo test
cargo build --release
./target/release/fastatools -h

 target/release/fastatools get-entry '/home/chase/Downloads/temp/aa/data' '/home/chase/Downloads/temp/aa/data.index' UjF_YxTKlIX4Qlt8DFeHycfpVbXQJO7C
```


