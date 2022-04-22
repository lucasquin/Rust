/// Create date: 21/04/2022
/// Project: Padaria Lucas Lanches
/// by Lucas Lopes Quintino

<<<<<<< HEAD
// Used to hide alerts for unused variables
# [allow(unused_variables)]

// Used to hide alerts of changing values of variables that will not be used later.
# [allow(unused_assignments)]

// Used to disable snake case
# [allow(non_snake_case)]
=======

// Used to hide alerts for unused variables
#[allow(unused_variables)]

// Used to hide alerts of changing values of variables that will not be used later.
#[allow(unused_assignments)]

// Used to disable snake case teste
#[allow(non_snake_case)]

use std::io::{self, Write};
>>>>>>> 557bdba (Adicionado itens da padaria')

fn main() {
    
    let (pao, pao_de_queijo, bolo, suco, leite, cafe, rosca) = ("Pão", "Pão de Queijo", "Bolo", "Suco", "Leite", "Cafe", "Rosca");
    
    // Used to clean screen
<<<<<<< HEAD
    //print!("{esc}c", esc = 27 as char);
    println!("--------------------------------------------");
    println!("-- Bem vindos a padaria do Lucas Lanches! --");
    println!("--------------------------------------------");
    println!("Faça teu pedido ai men");
=======
    // print!("{esc}c", esc = 27 as char);
    
    println!(" ------------------------------------------");
    println!("-- Bem vindos a padaria do Lucas Lanches! --");
    println!(" ------------------------------------------");
    println!();
    println!(" 1 - {}", pao);
    println!(" 2 - {}", pao_de_queijo);
    println!(" 3 - {}", bolo);
    println!(" 4 - {}", suco);
    println!(" 5 - {}", leite);
    println!(" 6 - {}", cafe);
    println!(" 7 - {}", rosca);
    println!();
   
    print!("Item: ");
    io::stdout().flush().unwrap(); 
    
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Erro ao ler a entrada do usuário.");
    let item:i32 = x.trim().parse().expect("A conversão de número falhou.");
    
    let mut v = 0;
    match item {
        1 => println!("teste"),
        _ => println!("Item não encontrado!")
    };


    //print!("Quantidade: ");
    //print!("Deseja finalizar o pedido? S/N: ");
>>>>>>> 557bdba (Adicionado itens da padaria')
}
