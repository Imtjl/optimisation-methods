use super::Function;

pub fn chord(mut a: f64, mut b: f64, eps: f64, func: &dyn Function) -> f64 {
    loop {
        let f_a = func.f_deriv(a);
        let f_b = func.f_deriv(b);
        let x = a - f_a * (a - b) / (f_a - f_b);

        let f_x = func.f_deriv(x);
        if f_x.abs() <= eps {
            return x;
        }

        if f_x > 0.0 {
            b = x;
        } else {
            a = x;
        }
    }
}
