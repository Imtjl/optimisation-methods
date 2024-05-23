struct MyFunction;

impl MyFunction {
    fn f(&self, x: f64, y: f64) -> f64 {
        5.0 * x.powi(2) + 4.0 * x * y + y.powi(2) - 16.0 * x - 12.0 * y
    }

    fn grad(&self, x: f64, y: f64) -> (f64, f64) {
        let dx = 10.0 * x + 4.0 * y - 16.0;
        let dy = 4.0 * x + 2.0 * y - 12.0;
        (dx, dy)
    }
}

fn gradient_descent(
    func: &MyFunction,
    x0: f64,
    y0: f64,
    alpha: f64,
    eps: f64,
    max_iter: usize,
) -> (f64, f64) {
    let mut x = x0;
    let mut y = y0;
    let mut alpha = alpha;
    let mut f_prev = func.f(x, y);

    for i in 0..max_iter {
        // Вычисляем градиент функции в текущей точке (x, y)
        let (dx, dy) = func.grad(x, y);

        // Вычисляем новые значения x и y, делая шаг в направлении, противоположном градиенту
        let x_new = x - alpha * dx;
        let y_new = y - alpha * dy;

        // Вычисляем значения функции в текущей и новой точках
        let f_new = func.f(x_new, y_new);

        // Выводим информацию о текущей итерации
        println!(
            "Итерация {}: (x, y) = ({:.6}, {:.6}), f(x, y) = {:.6}, градиент = ({:.6}, {:.6}), alpha = {:.6}, func = ({:.6}, {:.6})",
            i, x, y, f_prev, dx, dy, alpha, f_prev, f_new
        );

        // Проверяем, улучшилось ли значение функции
        if f_new < f_prev {
            // Если новое значение лучше, обновляем x и y
            x = x_new;
            y = y_new;
            f_prev = f_new;
        } else {
            // Если новое значение хуже, уменьшаем размер шага и переходим к следующей итерации
            alpha /= 2.0;
            continue;
        }

        // Проверяем условие остановки алгоритма
        if alpha < eps {
            println!("Размер шага стал меньше заданной точности на итерации {}", i);
            break;
        }
    }

    (x, y)
}

fn steepest_descent(func: &MyFunction, x0: f64, y0: f64, eps: f64, max_iter: usize) -> (f64, f64) {
    let mut x = x0;
    let mut y = y0;
    let mut iter = 0;

    loop {
        // Вычисляем градиент функции в текущей точке (x, y)
        let (dx, dy) = func.grad(x, y);

        // Вычисляем оптимальный шаг alpha методом одномерной минимизации
        let alpha = line_search(func, x, y, dx, dy, eps);

        // Выводим информацию о текущей итерации
        println!(
            "Итерация {}: (x, y) = ({:.6}, {:.6}), градиент = ({:.6}, {:.6}), alpha = {:.6}, func = {:.6}",
            iter, x, y, dx, dy, alpha, func.f(x, y)
        );

        // Вычисляем новые значения x и y, делая шаг в направлении, противоположном градиенту
        let x_new = x - alpha * dx;
        let y_new = y - alpha * dy;

        // Проверяем условие остановки алгоритма
        if (x_new - x).abs() < eps && (y_new - y).abs() < eps {
            println!("Достигнута заданная точность на итерации {}", iter);
            break;
        }

        // Обновляем значения x и y для следующей итерации
        x = x_new;
        y = y_new;
        iter += 1;

        if iter >= max_iter {
            println!("Достигнуто максимальное число итераций");
            break;
        }
    }

    (x, y)
}

fn line_search(func: &MyFunction, x: f64, y: f64, dx: f64, dy: f64, eps: f64) -> f64 {
    let mut a = 0.0;
    let mut b = 1.0;
    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;

    let mut x1 = b - (b - a) / phi;
    let mut x2 = a + (b - a) / phi;

    while (b - a) > eps {
        let f1 = func.f(x - x1 * dx, y - x1 * dy);
        let f2 = func.f(x - x2 * dx, y - x2 * dy);

        if f1 < f2 {
            b = x2;
            x2 = x1;
            x1 = b - (b - a) / phi;
        } else {
            a = x1;
            x1 = x2;
            x2 = a + (b - a) / phi;
        }
    }

    (a + b) / 2.0
}

fn main() {
    let func = MyFunction;
    let x0 = 0.0;
    let y0 = 0.0;
    let alpha = 0.1;
    let eps = 1e-6;
    let max_iter = 100;

    println!("### Градиентный спуск ###");
    println!("Начальная точка: (x0, y0) = ({}, {})", x0, y0);
    println!("Скорость обучения (величина шага alpha): {}", alpha);
    println!("Точность (eps): {}", eps);
    println!("Максимальное число итераций: {}", max_iter);
    println!();

    let (x_min, y_min) = gradient_descent(&func, x0, y0, alpha, eps, max_iter);
    let f_min = func.f(x_min, y_min);

    println!();
    println!("Минимум функции: ({:.6}, {:.6})", x_min, y_min);
    println!("Значение функции в минимуме: {:.6}", f_min);

    println!("### Наискорейший спуск ###");
    println!("Начальная точка: (x0, y0) = ({}, {})", x0, y0);
    println!("Точность (eps): {}", eps);
    println!("Максимальное число итераций: {}", max_iter);
    println!();

    let (x_min, y_min) = steepest_descent(&func, x0, y0, eps, max_iter);
    let f_min = func.f(x_min, y_min);

    println!();
    println!("Минимум функции: ({:.6}, {:.6})", x_min, y_min);
    println!("Значение функции в минимуме: {:.6}", f_min);
}
