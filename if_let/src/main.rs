enum Coin {
    Penny, Nickel, Dime, Quarter(usize) 
}

#[derive(Debug)]
enum Mensagem {
    Quit, Move { x: i32, y: i32}, Write(String), ChangeColor(i32, i32, i32)
}

fn main() {

    let config_max = Some(3);

    // using match
    match config_max {
        Some(max) => println!("match, the maximus is configured to be {}", max),
        _ => ()
    }

    // using if let 
    if let Some(max) = config_max {
        println!("if_let, the maximum is configured to be {}", max);
    } 

    let coin = Coin::Quarter(2600);
    let mut count = 0;

    match coin {
        Coin::Quarter(year) => println!("match, year of quarter {:?}!", year),
        _ => count += 1,
    }

    if let Coin::Quarter(year) = coin {
        println!("if let, year of quarter {:?}!", year);
    } else {
        count += 1;
    }

    //when only one value interest us
    let m1 = Mensagem::Write(String::from("hello"));
    let m2 = Mensagem::ChangeColor(0, 0, 0);

    if let Mensagem::Write(txt) = m1 {
        println!("The text in WRITE m1 is {}", txt);
    }

    if let Mensagem::Write(txt) = m2 {
        println!("The text in WRITE m2 is {}", txt);
    }

    //copy and move semantic
    let m3 = Mensagem::Write(String::from("hellohello"));

    match m3 {
        Mensagem::Write(txt) => println!("In m3 we have: {}", txt),
        _ => ()
    }


}
