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
# rgb hex codes
oklabby average 000 ffffff ...

# or use oklab floats directly
oklabby average "[0.0, 0.5, 1.0]" "[0,0,0]" ...
```

#### Quantize N steps between colors (inclusive)

![image](https://github.com/ozwaldorf/oklabby/assets/8976745/26638af3-6df0-47a6-a6c9-07df644af348)

```bash
oklabby quantize -s16 fff "[0,0,0]"
```

#### Generate shades using quantize

![image](https://github.com/ozwaldorf/oklabby/assets/8976745/ac8d0359-feae-4e7d-804e-8930904efde2)

```bash
oklabby quantize -s9 000 00f fff
```
