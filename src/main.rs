use std::io;

enum Roots
{
    None,
    One(f64),
    Two(f64, f64),
    Infinite,
}

fn main()
{
    println!("Hello! This program solves equations of form ax^2 + bx + c = 0.");
    println!("Tell me the coefficients!");
    
    let mut input_string = String::new();

    let mut successful_read = false;

    let mut coefficients = vec![0f64, 0f64, 0f64];

    while !successful_read
    {
        let input_result = io::stdin().read_line(&mut input_string);
        match input_result
        {
            Ok(_) => (),
            Err(error) => 
            {
                println!("{error}! Try again!"); 
                input_string.clear()
            },
        }
        let coefficient_result = input_string
                                                                     .split_whitespace()
                                                                     .map(|x| x.parse::<f64>());

        let mut i = 0usize;
        for c_res in coefficient_result
        {
            if i >= coefficients.len()
            {
                println!("Too many inputs, try again!");
                input_string.clear();
                i += 1;
                break;
            }

            match c_res
            {
                Ok(value) => { coefficients[i] = value; i += 1 },
                Err(error) => 
                {
                    println!("{error}! Try again!"); 
                    input_string.clear();
                    break
                },
            }
        }

        if i == coefficients.len()
        {
            successful_read = true;
        }
    }

    let a = coefficients[0];
    let b = coefficients[1];
    let c = coefficients[2];

    let roots = solve_quadratic(&a, &b, &c);

    match roots
    {
        Roots::None => println!("No roots :("),
        Roots::One(root) => println!("The root is {root}!"),
        Roots::Two(root1, root2) => println!("The roots are {root1}, {root2}!"),
        Roots::Infinite => println!("There are infinite roots!"),
    }
}

fn solve_linear(b_coefficient: &f64, c_coefficient: &f64) -> Roots
{
    if float_equal(b_coefficient, 0f64)
    {
        if float_equal(c_coefficient, 0f64)
        {
            return Roots::Infinite;
        }
        return Roots::None;
    }
    Roots::One(-c_coefficient / b_coefficient)
}

fn solve_quadratic(a_coefficient: &f64, b_coefficient: &f64, c_coefficient: &f64) -> Roots
{
    if float_equal(a_coefficient, 0f64)
    {
        return solve_linear(b_coefficient, c_coefficient);
    }

    let mut a = *a_coefficient;
    let mut b = *b_coefficient;
    let mut c = *c_coefficient;

    if a < 0f64
    {
        a = -a;
        b = -b;
        c = -c;
    }

    let mut discr = b * b - 4f64 * a * c;

    if discr < 0f64
    {
        return Roots::None;
    }

    if float_equal(&discr, 0f64)
    {
        return Roots::One(-b / (2f64 * a));
    }

    discr = discr.sqrt();

    let root1 = (-b - discr) / (2f64 * a);
    let root2 = (-b + discr) / (2f64 * a);

    Roots::Two(root1, root2)
}

fn float_equal(x1: &f64, x2: f64) -> bool
{
    let tolerance = 1e-6;
    f64::abs(x1 - x2) < tolerance
}
