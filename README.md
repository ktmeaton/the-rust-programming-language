# The Rust Programming Language

## Conda

The environment provides core Rust components as well as developer accessories.

```bash
mamba env create -f environment.yaml
```

## Lint

1. Install `clippy` and `rustfmt`.

    ```bash
    rustup component add clippy rustfmt miri

    # or with conda
    # note, clippy doesn't install with conda?
    cargo install rustfmt miri
    ```


    How about `miri`?

    ```bash
    cargo install miri
    ```

1. Format Rust code.

    ```bash
    cargo fmt
    ```

1. Lint Rust code.

    ```bash
    cargo clippy
    ```
