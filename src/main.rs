use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Devine le nombre");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Entre ton nombre : ");
        let mut gess = String::new();
        io::stdin().read_line(&mut gess).expect("erreur");
        println!("TU as dis {gess} ou je me trompe?");
        println!("le nombre secret est {secret_number}, tu as surement perdu");
        let gess: u32 =match gess
            .trim()
            .parse(){
                Ok(num)=>num,
                Err(_)=>continue,
            };
        match gess.cmp(&secret_number) {
            Ordering::Less => println!("trop petit"),
            Ordering::Greater => println!("trop grand"),
            Ordering::Equal => {println!("exact !!!");
        break;},
        }
    }
    let x =4;

}
