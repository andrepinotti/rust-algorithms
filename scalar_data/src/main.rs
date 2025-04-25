/*

Tabela 1.1: Tipos Inteiros em Rust

Comprimento	 Com Sinal	Sem Sinal
8   bits	    i8	       u8
16  bits	    i16	       u16
32  bits	    i32	       u32
64  bits	    i64	       u64
128 bits	    i128	   u128

i32 é o padrão ´para inteiros 
Existem dois tipos de pontos flutuantes: f32 e f64

*/

//Velocidade máxima de metros por segundo
const VELOCIDADE_MAXIMA: f64 = 200.0  * (1000.0 / 3600.0);

fn main() {
    
    println!("*****Início do programa*****");

    let chassi = 123456;
    let acel_min = 3.0;
    let acel_max = -10.0;
    let vel_max = VELOCIDADE_MAXIMA;
    let comprimento = 4;
    let posicao_atual = -100.0;
    let vel_atual = 0.0;
    let acel_atual = 0.0;

    //Adição
    let soma = posicao_atual + 10.0;

    //Subtração
    let difference = vel_atual - 10.0;

    //multiplicação
    let product = comprimento * 2;

    //divisão
    let quotient = acel_atual / 2.0;
    let floored = 2 / 3;

    let remainder = 43 % 5;

    let xxx: f64 = 123.55;
    // let yyy = xxx + 88;

    println!("trunc {}, round {}, ceil {}, floor {}",
        xxx.trunc(), xxx.round(), xxx.ceil(), xxx.floor()
    );  

    //Abaixo faremos algumas observações com char e booleanos

    let t = true;
    let f = false;

    let x = t && f;
    let y = t || !f;
    let z = 12 > 13;

    println!("Valor de X: {x} \nValor de Y: {y} \nValor de Z: {z}");

    println!("Até mais!");

}
