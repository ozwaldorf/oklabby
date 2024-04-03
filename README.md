# Oklabby

Simple cli tool for oklab color manipulation

## Usage

### Average N colors

```bash
oklabby average 000 fff ...
```

### Quantize N steps between colors (inclusive)

```bash
oklabby quantize -s16 000 fff ...
```

## Install

### Local

```bash
git clone https://github.com/ozwaldorf/oklabby
cargo install --path .
```

### Nix

A nix flake is available and can be run easily with:

```bash
nix run github:ozwaldorf/oklabby
```

Cache is provided via https://garnix.io
