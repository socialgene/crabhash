crabhash

```sh
git clone https://github.com/socialgene/crabhash.git

cd crabhash

cargo build --release 

./target/release/hash_proteins \
    '/glob/pattern/to/match/fasta/files' \
    '/output/directory/' \
    ncpus
```
