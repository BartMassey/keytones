/// Code by Google Gemini 2.5 Flash 2025-06-09.

// Use the `powi` method for f64 from the standard library
use std::f64;

// Chebyshev coefficients for the order-4 approximation of 2^((x+1)/2)
// over x in [-1, 1]. These coefficients are derived from chebfit.
// IMPORTANT: The first coefficient (c0) from numpy.polynomial.chebyshev.chebfit
// is effectively c0/2 for the general Clenshaw recurrence.
// We are doubling it here to use it in a direct sum, where T0(x) = 1.
const CHEBYSHEV_COEFFS_F64: [f64; 5] = [
    1.2071067811865475 * 2.0, // c0 - Doubled for direct summation
    0.6931471805599453,        // c1
    0.1732867951399863,        // c2
    0.028965902039997787,      // c3
    0.0036186000399998246,     // c4
];

/// Computes the 0-th order Chebyshev polynomial T0(x) = 1.
fn t0(x: f64) -> f64 {
    // T0(x) = 1
    let _ = x; // x is not used, but kept for consistent function signature
    1.0
}

/// Computes the 1st order Chebyshev polynomial T1(x) = x.
fn t1(x: f64) -> f64 {
    // T1(x) = x
    x
}

/// Computes the 2nd order Chebyshev polynomial T2(x) = 2x^2 - 1.
fn t2(x: f64) -> f64 {
    // T2(x) = 2x^2 - 1
    2.0 * x.powi(2) - 1.0
}

/// Computes the 3rd order Chebyshev polynomial T3(x) = 4x^3 - 3x.
fn t3(x: f64) -> f64 {
    // T3(x) = 4x^3 - 3x
    4.0 * x.powi(3) - 3.0 * x
}

/// Computes the 4th order Chebyshev polynomial T4(x) = 8x^4 - 8x^2 + 1.
fn t4(x: f64) -> f64 {
    // T4(x) = 8x^4 - 8.0 * x^2 + 1
    8.0 * x.powi(4) - 8.0 * x.powi(2) + 1.0
}

/// Evaluates an order-4 Chebyshev polynomial approximation of 2^(n/12)
/// by directly summing the products of coefficients and Chebyshev polynomials.
///
/// # Arguments
/// * `n` - An integer input in the range 0 to 12.
///
/// # Returns
/// An f64 result representing 2^(n/12).
pub fn evaluate_power_n_div_12_direct_f64(n: u8) -> f64 {
    // Ensure n is within the valid range
    if n > 12 {
        panic!("Input n must be between 0 and 12");
    }

    // Transform n from [0, 12] to x in [-1, 1] for Chebyshev polynomial evaluation.
    // x = (2 * n - 12) / 12
    let n_f64 = n as f64;
    let x = (2.0 * n_f64 - 12.0) / 12.0;

    // --- Diagnostic Prints ---
    println!("\n--- Direct F64 Evaluation for n={} ---", n);
    println!("Calculated x value: {:.15}", x); // Print x with high precision

    let t_funcs = [t0, t1, t2, t3, t4];
    let mut sum_terms = 0.0;

    for i in 0..5 {
        let coeff = CHEBYSHEV_COEFFS_F64[i];
        let t_val = t_funcs[i](x);
        let term = coeff * t_val;
        sum_terms += term;
        println!("Term {}: c_{} ({:.15}) * T_{}({:.15}) = {:.15} * {:.15} = {:.15}",
                 i, i, coeff, i, x, coeff, t_val, term);
    }
    println!("Final sum of terms: {:.15}", sum_terms);
    println!("------------------------------------");

    sum_terms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_f64_approximation() {
        // Expected values for 2^(n/12)
        let test_cases = [
            (0, 1.0),
            (1, 1.059463094),
            (2, 1.122462048),
            (3, 1.189207115),
            (4, 1.259921050),
            (5, 1.334839854),
            (6, 1.414213562), // sqrt(2)
            (7, 1.498307077),
            (8, 1.587401052),
            (9, 1.681792831),
            (10, 1.781797436),
            (11, 1.887748625),
            (12, 2.0),
        ];

        // Using a very tight tolerance for f64 as it should be highly accurate
        let tolerance = 1e-12; // 0.000000000001

        for (n, expected_val) in test_cases.into_iter() {
            let result_f64 = evaluate_power_n_div_12_direct_f64(n);
            // x_val already printed by the function itself
            println!("Test Result for n = {}: Expected = {:.10}, Got = {:.10}, Error = {:.12}\n",
                     n, expected_val, result_f64, (result_f64 - expected_val).abs());
            assert!((result_f64 - expected_val).abs() < tolerance,
                    "For n = {}, Expected: {}, Got: {}, Error: {}", n, expected_val, result_f64, (result_f64 - expected_val).abs());
        }
    }

    #[test]
    #[should_panic(expected = "Input n must be between 0 and 12")]
    fn test_out_of_range_n() {
        evaluate_power_n_div_12_direct_f64(13);
    }
}
