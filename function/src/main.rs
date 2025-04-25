fn exemplo_function(){
    println!("Exemplo de função que apenas imprime uma mensagem");
}

fn exemplo_com_parametro(x: i32){
    println!("O valor do parâmetro é {x}");
}

fn soma(x: f32, y: f32){
    println!("A soma é: {}", x + y)
}

//Quando a função tiver algum tipo de retorno temos que depois dos parênteses dos parâmetros colocar -> tipo do retorno  
//Se definirmos o "-> tipo" NÃO é necessário colocar o "return" 
fn somaret(x: i32, y: i32) -> i32{
    return x + y;
}

fn main() {
    
    exemplo_function();
    exemplo_com_parametro(55);
    soma(75.0, 88.4);
    somaret(33, 10);

    //Exemplos com bloco de código
    let bloco_x = {
        let x = 3;
        x+1
    };

    let bloco_y = {
        let y = 3;
        y+1;
    };

    println!("Valor do bloco de código sem ponto e vírgula: {:?}", bloco_x);    
    println!("Valor do bloco de código com ponto e vírgula: {:?}", bloco_y);

}   