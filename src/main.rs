extern crate num;
use num::Complex;

/// `limit` を繰り返しの上限として、`c` がマンデルブロ集合に含まれるかを判定する
///
/// `c` がマンデルブロ集合に含まれないなら `Some(i)` を返す
fn escape_time(c: Complex<f64>, limit: u32 -> Option<u32>) {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}
