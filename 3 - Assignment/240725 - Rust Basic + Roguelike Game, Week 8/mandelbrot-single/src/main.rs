use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder};
use num::complex::ComplexFloat;
use num::Complex;

use std::env;
use std::fs::File;
use std::str::FromStr;

/// Try to determine if `c` is in the Mandelbrot set,
/// using at most `limit` iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered
/// on the origin. If `c` seems to be a member (more precisely,
/// if we reached the iteration limit without being able to prove that
/// `c` is not a member), return `None`.
fn escape_time(_c: Complex<f64>, _limit: usize) -> Option<usize> {
    let mut x_i = 0.0;
    let mut y_i = 0.0;
    
    for i in 0.._limit {
        if (x_i * x_i + y_i*y_i)  > 4.0 {
            return Some(i);
        }
        let x_temp = x_i * x_i - y_i * y_i + _c.re;
        y_i = 2.0 * x_i * y_i + _c.im;
        x_i = x_temp;
    }
    return None;
}

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are
/// both strings that can be parsed by `T::from_str`.
///
/// If `s` has the proper form, return `Some<(x, y)>`.
/// If it doesn't parse correctly, return `None`.
fn parse_pair<T: FromStr>(_s: &str, _separator: char) -> Option<(T, T)> {
    // validate the input string
    let pairs: Option<(T, T)> = split_input_pair(_s, _separator);         
    if pairs.is_none() {
        return None;
    }
    return pairs
}

fn split_input_pair<T: FromStr>(s: &str, seperator: char) -> Option<(T, T)> {
    //split str with seperator
    let parts: Vec<&str> = s.split(seperator).collect();
    if parts.len() != 2 {
        return None;
    }
    let parsed_parts: Option<(T, T)> = match (T::from_str(parts[0]), T::from_str(parts[1])) {
        (Ok(left), Ok(right)) => Some((left, right)),
        _ => None,
    };
    return parsed_parts;
}

/// Parse a pair of floating-point numbers separated by a comma as a complex number.
fn parse_complex(_s: &str) -> Option<Complex<f64>> {
    let parts: Option<(f64, f64)> = split_input_pair(_s, ',');
    if parts.is_none() {
        return None;
    }
    return Some(Complex { re: parts.unwrap().0, im: parts.unwrap().1 });
}

/// Given the row and column of a pixel in the output image,
/// return the corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex plane
/// designating the area our image covers.
fn pixel_to_point(
    _bounds: (usize, usize),
    _pixel: (usize, usize),
    _upper_left: Complex<f64>,
    _lower_right: Complex<f64>,
) -> Complex<f64> {
    // upper_left >> bound 0, 0
    // lower_right >> bounds n-1 n-1
    let width = (_lower_right.re() - _upper_left.re()).abs();
    let height =(_upper_left.im() - _lower_right.im()).abs();

    // 으으으으으으
    Complex {
        re: _upper_left.re() + (width / _bounds.0 as f64) * _pixel.1 as f64,
        im: _upper_left.im() - (height / _bounds.1 as f64) * _pixel.0 as f64,
    }
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-left
/// and lower-right corners of the pixel buffer.
fn render(
    _pixels: &mut [u8],
    _bounds: (usize, usize),
    _upper_left: Complex<f64>,
    _lower_right: Complex<f64>,
) {
    for i in 0.._bounds.1 {
        for j in 0.._bounds.0 {
            let point = pixel_to_point(_bounds, (i, j), _upper_left, _lower_right);
            _pixels[i * _bounds.0 + j] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`,
/// to the file named `filename`.
fn write_image(_filename: &str, _pixels: &[u8], _bounds: (usize, usize)) {
    let image_file = File::create(_filename).expect("Error creating image file");
    let encoder = PngEncoder::new(image_file);
    encoder.write_image(_pixels, _bounds.0 as u32, _bounds.1 as u32, ExtendedColorType::L8)
    .expect("Error writing image file");
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPER_LEFT LOWER_RIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1.00,0.20",
            args[0]
        );

        std::process::exit(-1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("Error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("Error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("Error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair::<i32>("400x600", 'x'), Some((400, 600)));
        assert_eq!(parse_pair::<f64>("1.0,0.5", ','), Some((1.0, 0.5)));
        assert_eq!(parse_pair::<i32>("400x600", ','), None);
        assert_eq!(parse_pair::<f64>("1.0,0.5", 'x'), None);
    }

    #[test]
    fn test_split_input_pair() {
        assert_eq!(split_input_pair::<i32>("400x600", 'x'), Some((400, 600)));
        assert_eq!(split_input_pair::<f64>("1.0,0.5", ','), Some((1.0, 0.5)));
        assert_eq!(split_input_pair::<i32>("400x600", ','), None);
        assert_eq!(split_input_pair::<f64>("1.0,0.5", 'x'), None);
    }
}