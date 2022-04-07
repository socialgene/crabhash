crabhash

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

***input_file.faa.gz**
```
>WP_188403107.1 hypothetical protein [Fictibacillus barbaricus]
MEQKETFFVTAKGDIQPLPTDDHVHYFEIQATYDEKQQIDHLFTQIHANNKQEGLDIFSPKRHFSESHAEYHRGKDSKLV
YELFRYIYLLGTEKTKREIEEMNVLPELFESSHTTIEKVK
>WP_188403108.1 hypothetical protein [Fictibacillus barbaricus]
MKLPVHNQPFIKADPFVLEKTEVISAVKDYLKREGYETEILEDIYGVNLAAENEYHTLLIAAQGNTSELQLLSHKYPATQ
NETNFDKLIVDLLKHHEKNPAKTLVLASPDTPLFRDKAENIKQALDDLGIVRFWVKENGSIEWE
```

And outputs:
***input_file.faa.gz.fasta**
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
