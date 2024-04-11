# Oklabby

Simple cli tool for oklab color manipulation

## Usage

### Install from source

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

### Examples

#### Average N colors

![image](https://github.com/ozwaldorf/oklabby/assets/8976745/e41bba07-f376-4ef1-9a68-ea48396e1aa7)

```bash
# rgb hex
oklabby average 000 ffffff ...

# oklab floats
oklabby average "[0.0, 0.5, 1.0]" "[0,0,0]" ...
```

#### Quantize N steps between colors (inclusive)

![image](https://github.com/ozwaldorf/oklabby/assets/8976745/26638af3-6df0-47a6-a6c9-07df644af348)

```bash
# rgb hex
oklabby quantize -s16 000 ffffff ...

# oklab floats
oklabby quantize "[0.0, 0.5, 1.0]" "[0,0,0]" ...
```
