### Background 

The hash method (*sha512t24u*) used is described here:
https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7714221/


### Use
```sh
git clone https://github.com/socialgene/crabhash.git

cd crabhash

cargo build --release 

./target/release/crabhash \
    'input_file.faa.gz' \
    '/output/directory/' 
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

**/output/directory/input_file.faa.gz.tsv**
```
POt5XgLIthEqNIqvXB1PiFGkhjgwufcK	WP_188403107.1
KFMMpfPfblX3Bbrhtk40mJ5rwFi40OfA	WP_188403108.1
```

