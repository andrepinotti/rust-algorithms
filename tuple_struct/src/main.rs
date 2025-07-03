#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

fn main() {

    let black = Color(0, 0, 0);
    let test = Point(1, 2, 3);

    let unit_struct = AlwaysEqual;

    println!("{:?}", black);
    println!("{:?}", test);
    println!("{:?}", unit_struct);

}
