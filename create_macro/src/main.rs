macro_rules! either {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        }
        else {
            $false_expr
        }
    }
}
// example of how to use macro
fn main() {
  let foo = 1;
  let bar = 2;
  either!(foo == bar => println!("it is true"); println!("it is false"));
  either!(foo != bar => println!("it is true"); println!("it is false"));
}