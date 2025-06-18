/*!
This crate implements functions that take a MIDI key number
(in the range 0 to 127 inclusive) and produce a note
frequency or period.

There are two versions of these routines: "exact" versions
with high precision and "approximate" versions. The
approximate versions are "accurate enough". They may be
slightly faster, and may take slightly less program memory —
neither of these has been tested, though.

The crate can be compiled `no_std` with
`--no-default-features`. Otherwise the `std` feature will be
used.
*/

#![no_std]

use microcheby::ChebyshevExpansion as C;
#[cfg(not(feature = "std"))]
pub use num_traits::float::*;

mod consts {
    include!(concat!(env!("OUT_DIR"), "/consts.rs"));
}

/// Directly computes the frequency for a given midi key value $k$,
/// using the formula
///    $$440 \cdot 2^{\frac{k - 69}{12}}$$
/// for $k$ in $[0..127]$.
///
/// # Examples
///
/// ```
/// # use keytones::key_to_frequency;
/// assert_eq!(key_to_frequency(69).round(), 440.0);
/// ```
///
/// # Panics
///
/// Panics if `key` is not in the range `0..=127`.
pub fn key_to_frequency(key: u8) -> f32 {
    assert!(key < 128);
    440.0 * f32::powf(2.0, (key as f32 - 69.0) / 12.0)
}

/// Directly computes the "unit period" for a given midi key value $k$,
/// the inverse of frequency. This has units of
/// $$\frac{\text{seconds}}{\text{cycle}}$$
/// and can thus be multiplied by a sample rate in
/// $$\frac{\text{samples}}{\text{second}}$$
/// to get a cycle period in samples.
///
/// # Examples
///
/// ```
/// # use keytones::key_to_period;
/// assert_eq!((key_to_period(69) * 440.0).round(), 1.0);
/// ```
///
/// # Panics
///
/// Panics if `key` is not in the range `0..=127`.
pub fn key_to_period(key: u8) -> f32 {
    1.0 / key_to_frequency(key)
}

fn key_to_params_top(key: u8) -> (u8, u8) {
    assert!(key < 128);
    let m = (key + 120 - 116) % 12;
    let o = 10 - (key + 12 - m) / 12;
    (m, o)
}

#[test]
fn test_key_to_params_top() {
    let tests: &[(u8, (u8, u8))] = &[
        (116, (0, 0)),
        (115, (11, 1)),
        (69, (1, 4)),
        (68, (0, 4)),
        (67, (11, 5)),
    ];
    for &(key, vals) in tests {
        assert_eq!(key_to_params_top(key), vals);
    }
}

/// Computes the approximate frequency for a given midi key
/// value using a formula involving a Chebyshev series. (See
/// the source code for details.) The accuracy is better
/// than 0.1¢
///
/// # Examples
///
/// ```
/// # use keytones::key_to_frequency_approx;
/// assert_eq!(key_to_frequency_approx(69).round(), 440.0);
/// ```
///
/// # Panics
///
/// Panics if `key` is not in the range `0..=127`.
pub fn key_to_frequency_approx(key: u8) -> f32 {
    let (m, o) = key_to_params_top(key);
    let approx = C::const_new(0.0, 4.0 / 11.0, consts::CHEBYSHEV_TOP_OCTAVE);
    let f = approx.eval_4(m as f32);
    let p = f32::powf(2.0, -(o as f32));

    f * p
}

#[cfg(test)]
mod test {
    fn matches(k: u8, f: fn(u8) -> f32, g: fn(u8) -> f32, prec: f32) -> bool {
        let x = f(k);
        let y = g(k);
        f32::abs(x - y) < prec * f32::min(x, y)
    }

    pub fn check(f: fn(u8) -> f32, g: fn(u8) -> f32, prec: f32) {
        for k in 0..127 {
            assert!(
                matches(k, f, g, prec),
                "{} {} {}",
                k,
                f(k),
                g(k),
            );
        }
    }
}

#[test]
fn test_key_to_frequency_approx() {
    test::check(key_to_frequency, key_to_frequency_approx, 0.001);
}

fn key_to_params_bottom(key: u8) -> (u8, u8) {
    assert!(key < 128);
    let m = key % 12;
    let o = key / 12;
    (m, o)
}

#[test]
fn test_key_to_params_bottom() {
    let tests: &[(u8, (u8, u8))] = &[
        (0, (0, 0)),
        (1, (1, 0)),
        (69, (9, 5)),
        (68, (8, 5)),
        (67, (7, 5)),
    ];
    for &(key, vals) in tests {
        assert_eq!(key_to_params_bottom(key), vals);
    }
}

/// Computes the approximate unit period for a given midi key
/// value using a formula involving a Chebyshev series. (See
/// the source code for details.) The accuracy is better
/// than 0.1%.
///
/// # Examples
///
/// ```
/// # use keytones::key_to_period_approx;
/// assert_eq!((1.0 / key_to_period_approx(69)).round(), 440.0);
/// ```
///
/// # Panics
///
/// Panics if `key` is not in the range `0..=127`.
pub fn key_to_period_approx(key: u8) -> f32 {
    let (m, o) = key_to_params_bottom(key);
    let approx = C::const_new(0.0, 4.0 / 11.0, consts::CHEBYSHEV_BOTTOM_OCTAVE);
    let f = approx.eval_4(m as f32);
    let p = f32::powf(2.0, -(o as f32));

    f * p
}

#[test]
fn test_key_to_period_approx() {
    test::check(key_to_period, key_to_period_approx, 0.001);
}
