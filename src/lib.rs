/// Directly computes the frequency for a given midi key value,
/// using the formula
///    $$440 \cdot 2**((key - 69)/12)$$
/// for $key$ in $[0..127]$$
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
    440.0 * f32::powf(2.0, (key - 69) as f32 / 12.0)
}

// From Python written by Google Gemini 2.5 Flash 2025-06-10
/**
Computes an approximation of $2**(n/12)$ for $n$ in
$[0..11]$.

The approximation evaluates a Chebyshev series $$P(x) =
sum(a_k * T_k(x))$$ at at $x = (n - 116) * (2 / 11) - 1$
using Clenshaw's algorithm. The series coefficients are
calculated using NumPy's `Chebyshev.fit()`. The accuracy
is about five significant digits at the given points.

This approach probably strictly worse than a direct
calculation using `pow()`, but is kept for reference
reasons.

# Panics

Panics if `n` is not in the range `0..=11`.
*/
#[allow(unused)]
fn octave_approx(n: u8) -> f32 {
    const A: [f32; 4] = [1.40884464, 0.44202539, 0.03495718, 0.0018473];
    const N: usize = 4 - 1;

    // Convert `n` to -1..1 for Chebyshev.
    assert!(n < 12);
    let x = n as f32 * (2.0 / 11.0) - 1.0;

    // This is b_{N+1} (or equivalent for recurrence).
    let mut b_k_plus_2 = 0.0;
    // This is b_N (or equivalent for recurrence).
    let mut b_k_plus_1 = A[N];

    // `k` is the index of the current coefficient `A[k]` being processed.
    let mut k = (N - 1);
    loop {
        // Calculate b_k using the recurrence relation.
        let b_k = A[k] + 2.0 * x * b_k_plus_1 - b_k_plus_2;

        // Shift the b values for the next iteration.
        b_k_plus_2 = b_k_plus_1;
        b_k_plus_1 = b_k;

        if k == 0 {
            break;
        }
        k -= 1;
    }
    b_k_plus_1 - x * b_k_plus_2
}

#[test]
fn test_octave_approx() {
    for n in 0..12 {
        let delta = octave_approx(n) - f32::powf(2.0, n as f32 / 12.0);
        assert!(delta.abs() < 0.0001);
    }
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

    let f = f32::powf(2.0, (116.0 - 69.0) / 12.0) * 440.0;
    let t = octave_approx(m) * f;
    let p = f32::powf(2.0, -(o as f32));

    t * p
}

#[test]
fn test_key_to_freq_approx() {
    fn matches(k: u8) -> bool {
        let x = key_to_freq(k);
        let y = key_to_freq_approx(k);
        f32::abs(x - y) < 0.001 * f32::min(x, y)
    }

    assert!(matches(69));
    assert!(matches(116));
}
