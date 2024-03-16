use super::Function;

pub fn golden_section(mut a: f64, mut b: f64, eps: f64, func: &dyn Function) -> f64 {
    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let mut x1 = a + (2.0 - phi) * (b - a);
    let mut x2 = a + (phi - 1.0) * (b - a);

    while (b - a) > 2.0 * eps {
        if func.f(x1) < func.f(x2) {
            b = x2;
            x2 = x1;
            x1 = a + (2.0 - phi) * (b - a);
        } else {
            a = x1;
            x1 = x2;
            x2 = a + (phi - 1.0) * (b - a);
        }
    }
    (a + b) / 2.0
}
