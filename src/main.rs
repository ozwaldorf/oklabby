use clap::{Parser, Subcommand};
use hex::encode;
use oklab::{oklab_to_srgb, srgb_to_oklab, Oklab};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Average a list of colors together
    Average {
        #[clap(required = true)]
        colors: Vec<String>,
    },
    /// Quantize steps between each pair of colors (in order).
    Quantize {
        #[clap(required = true)]
        colors: Vec<String>,
        #[arg(short, long, default_value_t = 8)]
        steps: usize,
    },
}

#[inline(always)]
fn parse_one(mut s: String) -> Oklab {
    // extend 3 character rgb
    if s.len() == 3 {
        let t = s.clone();
        s.push_str(&t);
    }

    // parse hex string into rgb
    let hex = s.trim_start_matches('#');
    let mut c = [0u8; 3];
    hex::decode_to_slice(hex, &mut c).expect("valid hex color");

    // convert to oklab
    srgb_to_oklab(c.into())
}

#[inline(always)]
fn parse(input: Vec<String>) -> Vec<Oklab> {
    input.into_iter().map(parse_one).collect()
}

#[inline(always)]
fn average(colors: &[Oklab]) -> Oklab {
    let mut t = colors.iter().fold(
        Oklab {
            l: 0.0,
            a: 0.0,
            b: 0.0,
        },
        |mut sum, i| {
            sum.l += i.l;
            sum.a += i.a;
            sum.b += i.b;
            sum
        },
    );

    let len = colors.len() as f32;
    t.l /= len;
    t.a /= len;
    t.b /= len;
    t
}

fn quantize(a: &Oklab, b: &Oklab, mut steps: usize) -> Vec<(Oklab, f32)> {
    steps -= 1;
    let a = &[a.l, a.a, a.b];
    let b = &[b.l, b.a, b.b];

    let mut colors = Vec::with_capacity(steps);

    let percent = 1.0 / steps as f32;
    for i in 0..=steps {
        let scalar = percent * i as f32;
        let t = interpolation::lerp(a, b, &scalar);
        colors.push((
            Oklab {
                l: t[0],
                a: t[1],
                b: t[2],
            },
            scalar,
        ))
    }
    colors
}

fn main() {
    let cmd = Args::parse().cmd;
    match cmd {
        Cmd::Average { colors } => {
            let avg = average(&parse(colors));
            let rgb: [u8; 3] = oklab_to_srgb(avg).into();
            println!("#{}\t{rgb:?}", encode(rgb));
        }
        Cmd::Quantize { colors, steps } => {
            let colors = parse(colors);
            for s in colors.windows(2) {
                for (color, scalar) in quantize(&s[0], &s[1], steps) {
                    let rgb: [u8; 3] = oklab_to_srgb(color).into();
                    println!("{scalar:.2}:\t#{}\t{rgb:?}", encode(rgb));
                }
            }
        }
    }
}
