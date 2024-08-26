# RustKatas

Katas in Rust language

## How to build and test

### Build

```bash
cargo build
```

#### Lint

```bash
cargo clippy
```

#### Format

```bash
cargo fmt
```

### Test

```bash
cargo test
```

#### Coverage

```bash
cargo tarpaulin --out Lcov
cargo tarpaulin --out Html
```

### Watch mode

```bash
cargo install cargo-watch
cargo watch -x check -x test -x run
```

### Mutation test

```bash
cargo install --locked cargo-mutants
```

```bash
cargo mutants
```

#### Documentation

<https://mutants.rs/welcome.html>
