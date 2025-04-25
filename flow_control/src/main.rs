fn main() {

    println!("*****Início do programa*****");

    let number = 7;

    if number >= 5{
        println!("A condição é verdadeira");
    } else {
        println!("A condição é falsa");
    }

    //cascata de ifs
    if number % 2 == 0 {
        println!("O número é divisível por 2");
    } else if number % 3 == 0 {
        println!("O número é divisível por 3");
    } else if number % 4 == 0 {
        println!("O número é divisível por 4");
    } else {
        println!("O número não é divisível por 2, 3 ou 4");
    }   

    let outro_number = if number == 0 { 0 } else { 99 };
    println!("O valor do outro número é {outro_number}");

    println!("*****Início do controle*****");
    controle();
    utilizando_loop();
}   

fn controle (){

    let mut number = 5;

    while number != 0 {
        println!("While {number}");
        number -= 1;
    }

    let meses = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho", "Julho", "Agosto",
    "Setembro", "Outubro", "Novembro", "Dezembro"];

    println!("*****Mostrando todos os meses do ano******");
    let mut i = 1;
    for mes in meses.iter() {
        println!("Mês {}: {}", i, mes);
        i += 1;
    }

    println!("     \nUsando Range");
    for number in 1..=4{ // Atenção: o "=" antes do último número representa a inclusão dele
        println!("{}", number);
    }

    println!("     \nRange reverso");
    for number in (1..4).rev(){
        println!("{}", number);
    }


}

fn utilizando_loop(){

    let mut i = 0;

    println!("*****Utilizando loop******");
    //Números primos
    loop {
        i+=1;
        if i % 2 == 0{
           continue; 
        }   
        println!("i: {i}");
        if i >= 10{
            break;
        }

    }

    let mut contagem = 0;
    //loops dentro de loops
    'meu_loop: loop {
        
        println!("contagem: {contagem}");
        let mut faltam = 100;
        loop {
            println!("faltam: {faltam}");

            if faltam == 97{
                break;
            }

            println!("contagem: {contagem}");
            if contagem == 2 {
                break 'meu_loop;
            }

            faltam -= 1;

        }

        println!("Incrementa contagem");
        contagem += 1;

    }

    println!("Contagem final: {contagem}");

}