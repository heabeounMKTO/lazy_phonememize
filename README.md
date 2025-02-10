### build status
![h](https://github.com/heabeounMKTO/lazy_phonememize/actions/workflows/rust.yml/badge.svg)
## why?
i need some sort of text to phonememize library in rust ,
im too lazy to think of a solution other than to bind to libespeak-ng 
(this project currently only works for mac and linux)
## make sure you have espeak build dependencies installed 

https://github.com/espeak-ng/espeak-ng/blob/master/docs/building.md#linux-mac-bsd

## build/run the example rust program
```
cargo build --release
cargo run --example lp_cli --release -- --input-text "this is a lazy input text"
```
