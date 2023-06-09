# crate_feature

## Create library

```sh
cargo new --lib crate_feature
```

## How to use

```toml
crate_feature = { version = "0.1.0", git = "https://github.com/prongbang/crate_feature.git", features = ["feature_add", "feature_minus"] }
```