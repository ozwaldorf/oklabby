use anstream::println;
use anstyle::RgbColor;
use clap::Parser;
use hex::encode;
use oklab::{oklab_to_srgb, srgb_to_oklab, Oklab};

#[derive(Parser)]
#[command(version, about)]
enum Oklabby {
    /// Average a list of colors together.
    Average {
        /// List of colors, either RGB8 hex `#000`, or oklab `[0.0, 0.5, 1.0]`
        #[clap(required = true)]
        colors: Vec<String>,
    },
    /// Quantize steps between each pair of colors (in order).
    Quantize {
        /// List of colors, either RGB8 hex `#000`, or oklab `[0.0, 0.5, 1.0]`
        #[clap(required = true)]
        colors: Vec<String>,
        /// Total number of steps to generate for each start and end colors (inclusive)
        #[arg(short, long, default_value_t = 8)]
        steps: usize,
    },
}

#[inline(always)]
fn parse_one(s: String) -> Oklab {
    // Parse Oklab color, ie [0.0, 0.0, 0.0]
    if s.starts_with('[') && s.ends_with(']') {
        let slices: Vec<f32> = s[1..s.len() - 1]
            .split(',')
            .map(|s| s.trim().parse().expect("Invalid oklab float"))
            .collect();
        let [l, a, b] = slices[..] else {
            panic!("Malformed Oklab float sequence: {s}");
        };
        return Oklab { l, a, b };
    }

    // Parse RGB hex codes, ie #000, #000000, 000, 000000
    let trimmed = s.trim_start_matches('#');
    let mut c = [0u8; 3];
    match trimmed.len() {
        3 => {
            // Extend 3 character hex colors
            let t: String = trimmed.chars().flat_map(|a| [a, a]).collect();
            hex::decode_to_slice(&t, &mut c).expect("Invalid RGB color")
        }
        6 => hex::decode_to_slice(trimmed, &mut c).expect("Invalid RGB color"),
        _ => panic!("Malformed RGB hex code: {s}"),
    }
    // convert to oklab
    srgb_to_oklab(c.into())
}

#[inline(always)]
fn parse(input: Vec<String>) -> Vec<Oklab> {
    input.into_iter().map(parse_one).collect()
}

#[inline(always)]
fn format_color(color @ Oklab { l, a, b }: Oklab) -> String {
    let rgb @ [r, g, bb]: [u8; 3] = oklab_to_srgb(color).into();
    format!(
        "{}#{}\trgb({r:3},{g:3},{bb:3})\toklab[{l}, {a}, {b}]{}",
        if color.l < 0.5 {
            RgbColor(255, 255, 255).on(RgbColor(r, g, bb))
        } else {
            RgbColor(0, 0, 0).on(RgbColor(r, g, bb))
        },
        encode(rgb),
        anstyle::Reset
    )
}

#[inline(always)]
fn average(colors: Vec<Oklab>) -> Oklab {
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

#[inline(always)]
fn quantize(a: &Oklab, b: &Oklab, steps: usize) -> Vec<(Oklab, f32)> {
    let percent = 1.0 / (steps - 1) as f32;
    let a = &[a.l, a.a, a.b];
    let b = &[b.l, b.a, b.b];
    (0..steps)
        .map(|i| {
            let scalar = percent * i as f32;
            let t = interpolation::lerp(a, b, &scalar);
            (
                Oklab {
                    l: t[0],
                    a: t[1],
                    b: t[2],
                },
                scalar,
            )
        })
        .collect()
}

fn main() {
    match Oklabby::parse() {
        Oklabby::Average { colors } => println!("{}", format_color(average(parse(colors)))),
        Oklabby::Quantize { colors, steps } => parse(colors)
            .windows(2)
            .flat_map(|s| quantize(&s[0], &s[1], steps))
            .for_each(|(color, scalar)| {
                println!("{:5.1}%\t{}", scalar * 100.0, format_color(color));
            }),
    }
}
