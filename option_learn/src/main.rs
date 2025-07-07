/* Option seria mais ou menos assim
 * <T> is a generic type parameter 
 * enum Option<z>{
 *      None,
 *      Some(T)
 * }
 * 
*/ 

fn sum_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn sum_option(x: Option<i32>, y: Option<i32>) -> Option<i32>{
    match (x, y) {
        (Some(i), Some(j)) => Some(i+j),
        (Some(i), None) => None,
        (None, Some(j)) => None,
        (None, None) => None
    }
}

fn main() {

    let number_55 = Some(55);
    let not_number:Option<i32> = None;
    let number_20 = Some(20);

    println!("Sum number {:?} with one: {:?}", number_55, sum_one(number_55));
    println!("Sum of {:?} and {:?}: {:?}", number_55, not_number, sum_option(number_55, not_number));
    println!("Sum of {:?} and {:?}: {:?}", number_20, number_55, sum_option(number_20, number_55));

}
