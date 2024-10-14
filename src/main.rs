fn poly_division(dividend: Vec<f64>, divisor: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    // Degree of the dividend and divisor
    let mut quotient: Vec<f64> = vec![0.0; dividend.len() - divisor.len() + 1];
    let mut remainder: Vec<f64> = dividend.clone();

    // While degree of remainder >= degree of divisor
    while remainder.len() >= divisor.len() {
        // Leading term of quotient (coefficient)
        let lead_term = remainder[0] / divisor[0];
        let degree_diff = remainder.len() - divisor.len();

        // Add the leading term to the quotient
        quotient[degree_diff] = lead_term;

        // Subtract (leading_term * divisor * x^degree_diff) from the remainder
        for i in 0..divisor.len() {
            remainder[i] -= lead_term * divisor[i];
        }

        // Remove leading zeroes from remainder
        while remainder.len() > 0 && remainder[0].abs() < 1e-10 {
            remainder.remove(0);
        }
    }

    (quotient, remainder)
}

fn main() {
    // Example polynomials:
    // Dividend: x^5+2x^3-7  -> [1.0,0.0,2.0,0.0,0.0,-7.0]
    // Divisor: x^2 +4x-2  -> [1.0,4.0, -2.0]

    let dividend = vec![1.0,0.0,2.0,0.0,0.0,-7.0];
    let divisor = vec![1.0,4.0, -2.0];

    let (quotient, remainder) = poly_division(dividend, divisor);

    println!("Quotient: {:?}", quotient);
    println!("Remainder: {:?}", remainder);
}
