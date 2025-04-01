# wasm-fasta

A wasm wrapper for noodles providing fasta features for compressing and creating a index.

Project template taken from https://github.com/trunk-rs/trunk/tree/main/examples/vanilla

Todo
- [ ] Gain a understanding of Noodles
- [ ] Create a method that compresses a provided fasta file
- [ ] Create a method that generates a index for a provided fasta file
- [ ] Create a interface to the above methods
- [ ] Create a HTML example application
- [ ] Add CI/CD to deploy example page
- [ ] Add more useful information here

## Improvements
- Pass file to WASM so it can create a file reader and iterate over the file rather than loading it into memory

## Building

```
trunk build
```

## Testing

```
trunk serve
```