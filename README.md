## Overview 
This Rust program performs polynomial division. It divides two polynomials, represented as vectors of floating-point numbers, and outputs both the quotient and the remainder of the division. The program is designed to handle polynomials with real number coefficients and can handle polynomials of any degree.
## Key Features:
- **Polynomial Division:** Given a dividend and divisor, the program calculates the quotient and remainder using long division.
- **Handling Floating-Point Numbers:** Coefficients are represented as f64 values, allowing flexibility in dealing with real-number coefficients.
- **Leading Zero Removal:** After each division step, the program automatically removes leading zeros from the remainder.

## Functions
- `fn poly_division(dividend: Vec<f64>, divisor: Vec<f64>) -> (Vec<f64>, Vec<f64>)`
- This function takes two polynomials as input — `dividend` and `divisor` — and returns two vectors:
  - Quotient: The result of the division.
  - Remainder: The remainder left after dividing the two polynomials.
- Steps:
  - Initialize Quotient and Remainder:
    - The quotient is initialized to a vector of zeros, with the size determined by the degree difference between the dividend and divisor.
    - The remainder starts as a copy of the dividend.
  - Division Process:
    - The algorithm iteratively computes the leading term of the quotient and subtracts the corresponding multiple of the divisor from the remainder.
  - Remove Leading Zeros:
    - After each step, leading zeros in the remainder are removed to simplify the result.
  - Return:
    - The function returns a tuple (`quotient`, `remainder`).
- `fn main()`
  - This function serves as the entry point for the program. It defines two example polynomials for division:
    - Dividend: $x^5+2x^3-7$
    - Divisor: $x^2+4x-2$
  - The division is performed, and the quotient and remainder are printed.
