# About r-increment-cargo-version

Increment build number in Cargo.toml.

# Note

* Difficult to use in GitHub Actions.
* It could be used in Git hooks.

# Getting Started

```sh
wget https://github.com/mass10/r-increment-cargo-version/releases/latest/download/r-increment-cargo-version
```

```sh
./r-increment-cargo-version
```

# Usage

```sh
grep version Cargo.toml
```

    version = "0.1.4"

```sh
./r-increment-cargo-version
```

    [INFO] MATCHED for expression [\s*version\s*=\s*"(.*)"].
    [INFO] MATCHED for expression [(\d+)\.(\d+)\.(\d+)].
    [INFO] MATCHED for expression [\s*version\s*=\s*"(.*)"].
    [INFO] AFFECTED LINE:
            SRC [version = "0.1.4"]
            NEW [version = "0.1.5"]

```sh
grep version Cargo.toml
```

    version = "0.1.5"
