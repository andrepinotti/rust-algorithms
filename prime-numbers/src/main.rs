fn main() {

    let number = prime_numbers(13);

    println!("{}", if number {"Is prime."} else {"Isn't prime."});

}
 
fn prime_numbers(number: u32)-> bool {

    if number <= 1 {
        return false;
    } 

    let limite = (number as f64).sqrt() as u32;
    let mut d: u32 = 2;
    while d <= limite {
        if number % d == 0 {
            return false
        }
        d += 1;
    }

    true    
}