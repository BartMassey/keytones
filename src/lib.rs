pub fn key_to_freq(n: u8) -> f32 {
    assert!(n < 128);
    440.0 * f32::powf(2.0, (n - 69) as f32 / 12.0)
}

// From Python written by Google Gemini 2.5 Flash 2025-06-10
/**
Computes an approximate frequency in the "top octave" (MIDI
keys 116-127, G\#8-G9). This can be divided down to get the
rest of the MIDI keys.

The approximation evaluates a Chebyshev series $$P(x) =
sum(a_k * T_k(x))$$ at at $x = (n - 116) * (2 / 11) - 1$
using Clenshaw's algorithm. The series coefficients are
calculated using NumPy's `Chebyshev.fit()`. The accuracy
is about five significant digits at the given points.

This approach probably strictly worse than a direct
calculation using `pow()`, but is kept for reference
reasons.

# Panics

Panics if `n` is not in the range `116..=127`.
*/
#[allow(unused)]
fn top_octave_approx(n: u8) -> f64 {
    const A: [f64; 4] = [1.40884464, 0.44202539, 0.03495718, 0.0018473];
    const N: usize = 4 - 1;

    // Convert `n` to -1..1 for Chebyshev.
    assert!((116..=127).contains(&n));
    let x = (n - 116) as f64 * (2.0 / 11.0) - 1.0;

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

pub fn key_to_freq_approx(key: u8) -> u16 {
    let m = key % 12;
    let t = m + 115;
    let _f0 = top_octave_approx(t);
    let _p = key.leading_zeros();
    todo!()
}
