fn main() {

    let word = String::from("HELLO WORLD");

    println!("Slice 1: {}", &word[0..5]);
    println!("Slice 2: {}", &word[6..11]);
    println!("Slice 3: {}", &word[..2]);
    println!("Slice 4: {}", &word[3..]);
    println!("Slice 5: {}", &word[..]);

    /* NOTE -> RETURN  
        Slice 1: HELLO
        Slice 2: WORLD
        Slice 3: HE
        Slice 4: LO WORLD
        Slice 5: HELLO WORLD
    */


}
