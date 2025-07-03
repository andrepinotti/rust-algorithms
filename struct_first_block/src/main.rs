struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {

    println!("\n*****Valores do User1*****\n");

    let mut user1 = User {
        active: true,
        username: "André".to_string(),
        email: "amostradinho@gmail.com".to_string(),
        sign_in_count: 1 
    };

    println!("{:?}", user1.active);
    println!("{:?}", user1.username);
    println!("{:?}", user1.email);
    println!("{:?}", user1.sign_in_count);

    /* Alterando os valores */
    println!("\n*****Alterando Valores User1*****\n");
    user1.sign_in_count += 1;
    user1.active = false;
    user1.email = "teste@gmail.com".to_string();
    user1.username = "Gabriel".to_string();

    println!("{:?}", user1.active);
    println!("{:?}", user1.username);
    println!("{:?}", user1.email);
    println!("{:?}", user1.sign_in_count);

    println!("\n*****Passando por Build*****\n");
    
    let build  = build_user("pinottig".to_string(), "pinottig@yahoo".to_string());
    
    println!("{:?}", build.active);
    println!("{:?}", build.email);
    println!("{:?}", build.username);
    println!("{:?}", build.sign_in_count);


}


fn build_user(username: String, email: String) -> User {

    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1
    }

}