use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder};
use num::Complex;
use std::env;
use std::{fs::File, str::FromStr};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    // vec![v; n] は、長さ n のベクタを作り、v で初期化する
    let mut pixels = vec![0; bounds.0 * bounds.1];

    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;

    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
        .unwrap();
    }

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

/// `limit` を繰り返し回数の上限として、`c` がマンデルブロ集合に含まれるかを判定する。
///
/// `c` がマンデルブロ集合に含まれないなら `Some(i)` を返す。
/// `i` は `c` が原点を中心とする半径2の縁から出るまでにかかった繰り返し回数となる。
/// `c` がマンデルブロ集合に含まれているらしい場合 (正確に言うと、繰り返し回数の上限に達しても
/// `c` がマンデルブロ集合に含まれないことを示せなかった場合)、 `None` を返す。
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

/// 文字列 `s` は座標系のペア。`"400x600"`、 `1.0,0.5` など。
///
/// `s` は <left><sep><right> の形でなければならない。
/// <sep> は `separator` 引数で与えられる文字で、
/// <left> と <right> は双方とも `T::from_str` でパースできる文字列である。
/// `separator` は ASCII 文字でなければならない。
///
/// `s` が適切な形であれば `Some<(x, y)>` を返す。
/// パースできなければ `None` を返す。
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        // separator が s に出現しなかった場合
        None => None,
        // separator が s に出現した場合
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// カンマで分けられた浮動小数点のペアをパースして複素数を返す。
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// 出力される画像のピクセルの位置を取り、対応する複素平面上の点を返す。
///
/// `bounds` は出力画像の幅と高さをピクセル単位で与える。`pixel` は画像上の特定のピクセルを (行、列) ペアの形で指定する。
/// 仮引数 `upper_left`, `lower_right` は、出力画像に描画する複素平面を左上と右下で指定する。
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
        // ここが引き算となっているのはなぜか?上に動くと pixel.1 は増えるが、虚部は小さくなるからだ。
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}

/// 矩形範囲のマンデルブロ集合をピクセルのバッファに描画する。
///
/// 仮引数 `bounds` はバッファ `pixels` の幅と高さを指定する。
/// バッファ `pixels` はピクセルのグレースケール値をバイトで保持する。
/// `upper_left` と `lower_right` はピクセルバッファの左上と右下に対応する複素平面上の点を指定する。
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.1 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

/// 大きさが `bounds` で指定されたバッファ `pixels` を `filename` で指定されたファイルに書き出す
fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PngEncoder::new(output);
    encoder
        .write_image(
            pixels,
            bounds.0 as u32,
            bounds.1 as u32,
            ExtendedColorType::L8,
        )
        .map_err(std::io::Error::other)?;
    Ok(())
}
