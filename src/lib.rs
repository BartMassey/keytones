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
    1.0 / key_to_freq(key)
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
/// # use keytones::key_to_freq_approx;
/// assert_eq!(key_to_freq_approx(69).round(), 440.0);
/// ```
///
/// # Panics
///
/// Panics if `key` is not in the range `0..=127`.
pub fn key_to_freq_approx(key: u8) -> f32 {
    let (m, o) = key_to_params_top(key);
    let approx = C::const_new(0.0, 4.0 / 11.0, consts::CHEBYSHEV_TOP_OCTAVE);
    let f = approx.eval_4(m as f32);
    let p = f32::powf(2.0, -(o as f32));

    f * p
}

#[cfg(test)]
mod test {
    pub fn matches(k: u8, f: fn(u8) -> f32, g: fn(u8) -> f32, prec: f32) -> bool {
        let x = f(k);
        let y = g(k);
        f32::abs(x - y) < prec * f32::min(x, y)
    }
}

#[test]
fn test_key_to_freq_approx() {
    for k in 0..127 {
        assert!(
            test::matches(k, key_to_freq, key_to_freq_approx, 0.001),
            "{} {} {}",
            k,
            key_to_freq(k),
            key_to_freq_approx(k),
        );
    }
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
    for k in 0..127 {
        assert!(
            test::matches(k, key_to_period, key_to_period_approx, 0.001),
            "{} {} {}",
            k,
            key_to_period(k),
            key_to_period_approx(k),
        );
    }
}
