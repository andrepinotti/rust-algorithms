fn main() {

    println!("*******Fatorial Calculate!*******");

    let number: i64 = 5;
    // NOTE -> You choose between recursive and iter
    let result1 = fatorial_calculate_recursive(number);
    let result2 = fatorial_calculate_iterator(number);

    println!("Fatorial recursive of {}: {}", number, result1);
    println!("Fatorial iterator of {}: {}", number, result2);

}

fn fatorial_calculate_recursive(number: i64) -> i64 {

    if number <= 1{
        return 1;
    }

    number *  fatorial_calculate_recursive(number-1)

}

fn fatorial_calculate_iterator(number: i64) -> i64 {
    (1..=number).product()
}