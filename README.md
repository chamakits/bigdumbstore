# bigdumbstore
Reimplementation of the 'bigdumbstore' I began to do in C some time back, but with rust.

## Status [![Build Status](https://travis-ci.org/chamakits/bigdumbstore.svg?branch=master)](https://travis-ci.org/chamakits/bigdumbstore) [![Coverage Status](https://coveralls.io/repos/chamakits/bigdumbstore/badge.svg?branch=master&service=github)](https://coveralls.io/github/chamakits/bigdumbstore?branch=master) [![Circle CI](https://circleci.com/gh/chamakits/bigdumbstore.svg?style=svg)](https://circleci.com/gh/chamakits/bigdumbstore)

## What is it
It's a VERY dumb key value store. It given (from stdin) the value [value], and the key [key] it basically inserts at the end of the file:
[value][key][size of value 0 padded up to 999][size of key 0 padded up to 999]

Ex:

```
given [value] = value
given [key] = key

It writes at the end of the file:
valuekey005003
```

## Building

```bash 
cargo build --release

# For the test, will need to use a nightly build; used: cargo 1.83.0-nightly (80d82ca22 2024-09-27)
rustup install nightly
cargo +nightly build --release
```

## Writing

```
echo "this is a value" | ./target/release/bigdumbstore p a-key
```

## Reading

```
./target/release/bigdumbstore g a-key
> this is a value
```


# Useful links:

## Helping to get this working on appveyor
- https://github.com/japaric/rust-everywhere/blob/master/appveyor.yml
- https://github.com/jgallagher/rusqlite/blob/master/appveyor.yml