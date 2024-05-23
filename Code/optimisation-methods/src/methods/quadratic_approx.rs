use super::Function;

pub fn quadratic_approx(a: f64, b: f64, eps: f64, func: &dyn Function) -> f64 {
    let eps1 = eps;
    let eps2 = eps;
    let mut x1 = (b - a) / 2.0;
    let mut x2;
    let mut x3;
    let mut f1;
    let mut f2;
    let mut f3;
    let mut x_min;
    let mut f_min;
    let delta_x = (b - a) / 50.0;
    println!("Eps1: {}; Eps2: {}; delta_x: {}; x1: {};", eps1, eps2, delta_x, x1);

    let mut iteration = 1;

    loop {
        println!("Iteration {}", iteration);

        // Шаг 2
        x2 = x1 + delta_x;
        println!("Step 2: x2 = {}", x2);

        // Шаг 3
        f1 = func.f(x1);
        f2 = func.f(x2);
        println!("Step 3: f1 = {}, f2 = {}", f1, f2);

        // Шаг 4
        if f1 > f2 {
            x3 = x1 + 2.0 * delta_x;
        } else {
            x3 = x1 - delta_x;
        }
        println!("Step 4: x3 = {}", x3);

        // Шаг 5
        f3 = func.f(x3);
        println!("Step 5: f3 = {}", f3);

        // Шаг 6
        f_min = f1.min(f2).min(f3);
        x_min = if f_min == f1 {
            x1
        } else if f_min == f2 {
            x2
        } else {
            x3
        };
        println!("Step 6: f_min = {}, x_min = {}", f_min, x_min);

        // Шаг 7
        let denominator = (x2 - x3) * f1 + (x3 - x1) * f2 + (x1 - x2) * f3;
        if denominator == 0.0 {
            println!("Step 7: Denominator is zero, setting x1 = x_min = {}", x_min);
            x1 = x_min;
            iteration += 1;
            continue;
        }

        let x_ = 0.5 * ((x2 * x2 - x3 * x3) * f1 + (x3 * x3 - x1 * x1) * f2 + (x1 * x1 - x2 * x2) * f3) / denominator;
        let f_ = func.f(x_);
        println!("Step 7: x* = {}, f(x*) = {}", x_, f_);

        // Шаг 8
        if (f_min - f_).abs() / f_.abs() < eps1 && (x_min - x_).abs() / x_.abs() < eps2 {
            println!("Step 8: Conditions met, returning x* = {}", x_);
            return x_;
        }

        if x_ >= x1 && x_ <= x3 {
            println!("Step 8: x* is within [x1, x3]");
            if f_ < f_min {
                x_min = x_;
                f_min = f_;
                println!("Step 8: Updating x_min = {}, f_min = {}", x_min, f_min);
            }
            if x_ < x_min {
                x1 = x_;
                x3 = x_min;
            } else {
                x1 = x_min;
                x3 = x_;
            }
            println!("Step 8: New interval: [x1, x3] = [{}, {}]", x1, x3);
        } else {
            println!("Step 8: x* is outside [x1, x3], setting x1 = x* = {}", x_);
            x1 = x_;
        }

        iteration += 1;
    }
}
