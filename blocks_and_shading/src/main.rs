fn main() {

    println!("*****Início do Programa*****");
    let x = 5;
    println!("Primeiro x: {x}");
    let x = x + 1;
    println!("Novo valor de x: {x}");

    // Este é um exemplo de Bloco de Código, vamos pegar as mesmas variáveis e imprimi-las
    {
        let x = x *2;
        println!("Valor de x dentro do bloco: {x}");
    }   
    //Fora do bloco ele irá ignorar 
    println!("Valor de x fora do bloco: {x}");

    //Agora mostraremos redefinição de tipos de variáveis
    let spaces = "   ";
    println!("O valor de spaces é: {spaces}"); // ele literalmente vai mostrar nada

    let spaces_tamanho = spaces.len();
    //Já que estamos lidando com variáveis
    println!("O tamanho de spaces é: {spaces_tamanho}");

    // O que quero mostrar é que não podemos redeclarar a variável como um novo tipo, assim:
    // spaces = 5;
    // Rust é uma linguagem fortemente tipada, por isso devemos redeclará-la como let novamente 
    let spaces = 5;
    println!("Novo valor de spaces: {spaces}")

}
