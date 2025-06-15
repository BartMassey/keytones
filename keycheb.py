import numpy as np
from numpy.polynomial import Chebyshev

xdata = [(2 / 11) * n - 1 for n in range(12)]
ydata = [440 * 2**((n + 116 - 69) / 12) for n in range(12)]
poly = Chebyshev.fit(xdata, ydata, 3)
print(poly)
print(poly.coef)

def t(n, x):
    if n == 0:
        return 1
    elif n == 1:
        return x
    else:
        return 2 * x * t(n - 1, x) - t(n - 2, x)

def tpoly(c, x):
    n = len(c)
    return sum(c[i] * t(i, x) for i in range(n))

# Written by Google Gemini 2.5 Flash 2025-06-10
def clenshaw(a: np.ndarray, x: float) -> float:
    """
    Evaluates a Chebyshev series P(x) = sum(a_k * T_k(x)) at a given point x
    using Clenshaw's algorithm.

    This corrected implementation accurately reflects the standard Clenshaw
    recurrence for Chebyshev polynomials as used in NumPy, including the
    specific final calculation step.

    Args:
        x: The point at which to evaluate the series.
        a: A NumPy array of coefficients [a_0, a_1, ..., a_N].

    Returns:
        The value of the polynomial P(x) at point x.
    """
    N = len(a) - 1

    # Handle edge cases for very short series directly
    if N < 0:
        return 0.0  # An empty series sums to zero

    if N == 0:
        # P(x) = a_0 * T_0(x). Since T_0(x) = 1, P(x) = a_0.
        return float(a[0]) # Cast to float to ensure consistent return type

    if N == 1:
        # P(x) = a_0 * T_0(x) + a_1 * T_1(x).
        # Since T_0(x) = 1 and T_1(x) = x, P(x) = a_0 * 1 + a_1 * x.
        return float(a[0] + a[1] * x) # Cast to float

    # For N >= 2, apply the main Clenshaw recurrence.
    # Initialize the intermediate 'b' values based on standard Clenshaw:
    # b_{N+1} = 0
    # b_N = a_N
    
    # In our loop, b_k_plus_2 will hold b_{k+2} and b_k_plus_1 will hold b_{k+1}.
    # For the first iteration (k = N-1), these need to be b_{N+1} and b_N respectively.
    b_k_plus_2 = 0.0           # This is b_{N+1} (or equivalent for recurrence)
    b_k_plus_1 = float(a[N])   # This is b_N (or equivalent for recurrence)

    # Iterate backwards from k = N-1 down to 0.
    # 'k' is the index of the current coefficient 'a[k]' being processed.
    for k in reversed(range(N)): # range(N) goes from 0 to N-1, so reversed goes from N-1 down to 0
        # Calculate b_k using the recurrence relation: b_k = a_k + 2x * b_{k+1} - b_{k+2}
        b_k = float(a[k]) + 2.0 * x * b_k_plus_1 - b_k_plus_2

        # Shift the 'b' values for the next iteration:
        # The current b_k_plus_1 becomes the new b_k_plus_2.
        # The newly calculated b_k (which is b_k) becomes the new b_k_plus_1.
        b_k_plus_2 = b_k_plus_1
        b_k_plus_1 = b_k
    
    # *** CRITICAL CORRECTION HERE ***
    # For Chebyshev polynomials, the final result is b_0 - x * b_1.
    # After the loop:
    # b_k_plus_1 holds b_0
    # b_k_plus_2 holds b_1 (it held b_k_plus_1 from the iteration before the last)
    return b_k_plus_1 - x * b_k_plus_2

for n in range(116, 128):
    f = 440 * 2**((n - 69)/12)
    y = (n - 116) * 2 / 11 - 1
    print(n, f, poly(y), tpoly(poly.coef, y), clenshaw(poly.coef, y))
