# bigdumbstore
Reimplementation of the 'bigdumbstore' I began to do in C some time back, but with rust.

## Building

```
    cargo build --release
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
