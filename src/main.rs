extern crate num;
use num::Complex;
use std::str::FromStr;

#[allow(dead_code)]
/// `limit` を繰り返しの上限として、`c` がマンデルブロ集合に含まれるかを判定する
///
/// `c` がマンデルブロ集合に含まれないなら `Some(i)` を返す
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                // find(separator)した結果、区切り文字で分割してどちらも期待する型にマッチしてOだった場合
                (Ok(l), Ok(r)) => Some((l, r)),
                // 上記マッチパターンに入らなかったワイルドカードパターン_
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",        ','), None);
    assert_eq!(parse_pair::<i32>("10,",     ','), None);
    assert_eq!(parse_pair::<i32>(",10",     ','), None);
    assert_eq!(parse_pair::<i32>("10,20",   ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// 出力される画像のピクセル位置を取り、対応する複素平面上の点を返す。
/// `bounds` は出力画像の幅と高さをピクセル単位で与える。
/// `pixel` は画像上の特定ピクセルを (行, 列) ペアの形で指定する。
/// `upper_left` と `lower_right` は、出力画像に描画する複素平面を左上と右下で指定する。
fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>)
    -> Complex<f64>
{
    let (width, height) = (lower_right.re - upper_left.re,
                           upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 100), (25, 75),
                              Complex { re: -1.0, im:  1.0 },
                              Complex { re:  1.0, im: -1.0 }),
               Complex { re: -0.5, im: -0.5 });
}
