// 1) Faça uma função que receba um valor inteiro que represente a idade de uma pessoa e imprima a sua faixa etária. 
// Bebê: Recém-nascido até 2 anos de idade. Criança: 2 anos até 12 anos de idade. Adolescente: 13 anos até 19 anos de idade. 
// Jovem adulto: 20 anos até 29 anos de idade. Adulto jovem: 30 anos até 39 anos de idade. Adulto de meia-idade: 40 anos até 59 anos de idade. 
// Idoso: 60 anos em diante. Lembre: 13..19 inclui o 13, mas não o 19. 13..=19, inclui tanto o 13 como o 19.

enum Idade {
    Num(i32),
    Nenhum
}

fn main() {
    let novaIdade = Idade::Num(60);

    match novaIdade{
        Idade::Num(x) if x <= 2 => {
            println!("Faixa etaria: Bebe");
        },
        Idade::Num(x) if x <= 12 => {
            println!("Faixa etaria: Criança");
        },
        Idade::Num(x) if x <= 19 => {
            println!("Faixa etaria: Adolescente");
        },
        Idade::Num(x) if x <= 29 => {
            println!("Faixa etaria: Jovem Adulto");
        },
        Idade::Num(x) if x <= 39 => {
            println!("Faixa etaria: Adulto Jovem");
        },
        Idade::Num(x) if x <= 59 => {
            println!("Faixa etaria: Adulto de meia-idade");
        },
        Idade::Num(x) if x >= 60  => {
            println!("Faixa etaria: Idoso");
        },
        Idade::Nenhum => {
            println!("Opção vazia");
        }
        _=> {
            println!("Opção Inválida");
        }
    
    }
}
