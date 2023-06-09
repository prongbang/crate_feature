# crate_feature

## Create library

```sh
cargo new --lib crate_feature
```

## How to use

- Cargo.toml

```toml
crate_feature = { version = "0.1.0", git = "https://github.com/prongbang/crate_feature.git", feature = ["feature_add", "feature_minus"] }
```