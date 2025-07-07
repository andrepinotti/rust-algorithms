use std::{io::{self, Read}, option};

#[derive(Debug)]
enum TipoAluno{
    NaoBolsista,
    Bolsista(u32), 
}

#[derive(Debug)]
struct Aluno {
    nome: String,
    tipo: TipoAluno,
    nota:  f64
}

fn consulta_nota(nome: &str, turma: &Vec<Aluno>) -> Option<f64>{
    for a in turma {
        if nome == a.nome {
            return Some(a.nota);
        }
    }
    None
}

fn main() {
    let turma = vec![
        Aluno{ nome: "João".to_string(), tipo: TipoAluno::NaoBolsista, nota: 7.8},
        Aluno{ nome: "Maria".to_string(), tipo: TipoAluno::Bolsista(50), nota: 8.9 },
        Aluno{ nome: "Pedro".to_string(), tipo: TipoAluno::NaoBolsista, nota: 5.3 },
        Aluno{ nome: "Rafaela".to_string(), tipo: TipoAluno::Bolsista(100), nota: 9.9},
        Aluno{ nome: "Mário".to_string(), tipo: TipoAluno::NaoBolsista, nota: 30.5}
    ];

    println!("\n*****Turma****\n");
    println!("{:?}", turma);

    match turma[0].tipo {
        TipoAluno::Bolsista(x) => println!("{} é bolsista com {} de desconto", turma[0].nome,x),
        TipoAluno::NaoBolsista => println!("{} não é bolsista", turma[0].nome)
    }

    loop {

        println!("\nDigite o nome do aluno:\n");
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Ocorreu um erro não esperado");

        let option_name = input.trim().split_whitespace().nth(0);

        match option_name {
            Some(x) => println!("Some: leu '{}'", x),
            None => println!("None: não leu nada")
        }

        if let Some(nome) = option_name {
            println!("Nome: {}", nome);
        
            if let Some(nota) = consulta_nota(nome, &turma) {
                println!("Nota: {}", nota);
                match nota {
                    x if x < 4.0 => println!("Está reprovado"),
                    x if x > 4.0 && x < 6.0 => println!("Está de recuperação"),
                    x if x >= 6.0 && x <=10.0 => println!("Está aprovado"),
                    _ => panic!("Nota com valor inesperado: {}", nota)
                }
            } else {
                println!("Aluno não existe na base de dados");
            }
        
        } else {
            println!("Nenhum nome foi teclado!");
            break;
        }
    }


}
