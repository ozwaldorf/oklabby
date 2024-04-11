use anstream::println;
use anstyle::RgbColor;
use clap::Parser;
use hex::encode;
use oklab::{oklab_to_srgb, srgb_to_oklab, Oklab};

#[derive(Parser)]
#[command(version, about)]
enum Oklabby {
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
fn parse_one(s: String) -> Oklab {
    let trimmed = s.trim_start_matches('#');
    let mut c = [0u8; 3];
    match trimmed.len() {
        3 => {
            // Extend 3 character hex colors
            let mut t = String::with_capacity(6);
            t.push_str(trimmed);
            t.push_str(trimmed);
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
fn format_color(color: Oklab) -> String {
    let rgb @ [r, g, b]: [u8; 3] = oklab_to_srgb(color).into();
    format!(
        "{}#{}\trgb({r:3},{g:3},{b:3}){}",
        if color.l < 0.5 {
            RgbColor(255, 255, 255).on(RgbColor(r, g, b))
        } else {
            RgbColor(0, 0, 0).on(RgbColor(r, g, b))
        },
        encode(&rgb),
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
            .map(|s| quantize(&s[0], &s[1], steps))
            .flatten()
            .for_each(|(color, scalar)| {
                println!("{scalar:.2}:\t{}", format_color(color));
            }),
    }
}
