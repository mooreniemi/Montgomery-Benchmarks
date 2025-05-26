Requires nightly:

```
rustup toolchain install nightly
rustup override set nightly
```

Then you can do:

```
cargo run
```

Can also use `cargo-asm`:

```
cargo install cargo-asm

# count overall
for f in yuval_mult::scalar_mul vanilla_cios::scalar_mul optimised_cios::scalar_mul_unwrapped; do echo -n "$f: "; cargo asm minimal_mult::$f | grep -v '^$' | wc -l; done

# memory ops
for f in yuval_mult::scalar_mul vanilla_cios::scalar_mul optimised_cios::scalar_mul_unwrapped; do echo -n "$f: "; cargo asm minimal_mult::$f | grep -E 'mov|lea|push|pop' | wc -l; done
```