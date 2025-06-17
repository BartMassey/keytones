use microcheby::ChebyshevExpansion as C;

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
/// # use keytones::key_to_freq;
/// assert_eq!(key_to_freq(69).round(), 440.0);
/// ```
///
/// # Panics
///
/// Panics if `key` is not in the range `0..=127`.
pub fn key_to_freq(key: u8) -> f32 {
    assert!(key < 128);
    440.0 * f32::powf(2.0, (key as f32 - 69.0) / 12.0)
}

fn key_to_params(key: u8) -> (u8, u8) {
    assert!(key < 128);
    let m = (key + 120 - 116) % 12;
    let o = 10 - (key + 12 - m) / 12;
    (m, o)
}

#[test]
fn test_key_to_params() {
    let tests: &[(u8, (u8, u8))] = &[
        (116, (0, 0)),
        (115, (11, 1)),
        (69, (1, 4)),
        (68, (0, 4)),
        (67, (11, 5)),
    ];
    for &(key, vals) in tests {
        assert_eq!(key_to_params(key), vals);
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
/// # use keytones::key_to_freq_approx;
/// assert_eq!(key_to_freq_approx(69).round(), 440.0);
/// ```
///
/// # Panics
///
/// Panics if `key` is not in the range `0..=127`.
pub fn key_to_freq_approx(key: u8) -> f32 {
    let (m, o) = key_to_params(key);
    let approx = C::const_new(0.0, 4.0 / 11.0, consts::CHEBYSHEV_TOP_OCTAVE);
    let f = approx.eval_4(m as f32);
    let p = f32::powf(2.0, -(o as f32));

    f * p
}

#[test]
fn test_key_to_freq_approx() {
    fn matches(k: u8) -> bool {
        let x = key_to_freq(k);
        let y = key_to_freq_approx(k);
        f32::abs(x - y) < 0.001 * f32::min(x, y)
    }

    for k in 0..127 {
        assert!(
            matches(k),
            "{} {} {}",
            k,
            key_to_freq(k),
            key_to_freq_approx(k),
        );
    }
}
