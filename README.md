# The Rust Programming Language

## Conda

The environment provides core Rust components as well as developer accessories.

```bash
mamba env create -f environment.yaml
```

## Lint

1. Install `rustfmt`, `clippy`, and `miri`.

    ```bash
    rustup +nightly component add rustfmt clippy miri
    ```

1. Format Rust code.

    ```bash
    cargo fmt
    ```

1. Lint Rust code.

    ```bash
    cargo clippy
    ```
