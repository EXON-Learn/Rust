fn main() {
    println!("Hello, world!");
    println!("{}", function1());

    let x = 5;
    if x > 4 {
        println!("x is greater than 4");
    } else {
        println!("x is less than 4");
    }

    let mut number = 5;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40];
    for element in a.iter() {
        println!("{}", element);
    }

    for number in 1..=4 {
        println!("{}", number);
    }

    let f = factorial(4);
    println!("{}", f);
}

fn function1() -> i32 {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    x+y
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}
