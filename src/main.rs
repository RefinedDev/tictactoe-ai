// use std::io;
// use std::io::*;
extern crate rand;

use std::{thread, time, mem};
use rand::{thread_rng,Rng};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { println!("You need to pass your character! Either 'X' or 'O'"); return; }
    let chosen_character: String = args[1].clone();
    
    // print!("Choose a Character, either 'X' or 'O': ");
    // io::stdout().flush().unwrap();
    // let mut chosen_character = String::new();
    // io::stdin().read_line(&mut chosen_character).expect("Failed");

    if chosen_character == "help"{
        println!("How this works is. You have to pass a character first. Either 'X' or 'O'.\nAfter that when the game starts, you will see 3 types of characters '||' 'X' and 'O'.\nIf you chose 'X' your character is X and your opponent is O and vice versa.\n'||' means that, that area has not been filled with a character yet.");
        return;
    }else if (chosen_character.trim_end() != "X") && (chosen_character.trim_end() != "O") {
        println!("The character can only be either 'X' or 'O'");
        return;
    }
    
    let winner = start_ai(&chosen_character);
    println!("{}", winner);
}

pub fn start_ai(player_character: &String) -> &str
{
    let mut firstrow = vec![String::from("||"),String::from("||"),String::from("||")];
    let mut secondrow = vec![String::from("||"),String::from("||"),String::from("||")];
    let mut thirdrow = vec![String::from("||"),String::from("||"),String::from("||")];

    let winner: &str;
    let mut player_turn = true;
    let opponent_character = if player_character == "X" { "O" } else {"X"};

    println!("Starting!");

    thread::sleep(time::Duration::from_secs(2));

    loop {
        // BIG ASS HARDCODED IF STATEMENT TO CHECK IF PATTERNS ARE MATCHING WITH SAME CHARACTERS.

        // 1. CHECK IF PATTERN ARE MATCHING IN HORIZONTLY 
        let idk = vec![player_character, player_character, player_character];
        let idk2 = vec![opponent_character, opponent_character, opponent_character];

        if (vec![&firstrow[0], &firstrow[1], &firstrow[2]] == idk) || (vec![&secondrow[0], &secondrow[1], &secondrow[2]] == idk) || (vec![&thirdrow[0], &thirdrow[1], &thirdrow[2]] == idk){
            winner = "You're the winner!";
            break;
        } else if (vec![&firstrow[0], &firstrow[1], &firstrow[2]] == idk2) || (vec![&secondrow[0], &secondrow[1], &secondrow[2]] == idk2) || (vec![&thirdrow[0], &thirdrow[1], &thirdrow[2]] == idk2){
            winner = "You lost!";
            break;
        }
        // 2. CHECK IF PATTERN ARE MATCHING VERTICALLY
        else if (vec![&firstrow[0], &secondrow[0], &thirdrow[0]] == idk) || (vec![&firstrow[1], &secondrow[1], &thirdrow[1]] == idk) || (vec![&firstrow[2], &secondrow[2], &thirdrow[2]] == idk){
            winner = "You're the winner!";
            break;
        } else if (vec![&firstrow[0], &secondrow[0], &thirdrow[0]] == idk2) || (vec![&firstrow[1], &secondrow[1], &thirdrow[1]] == idk2) || (vec![&firstrow[2], &secondrow[2], &thirdrow[2]] == idk2){
            winner = "You lost!";
            break;
        } 
        // 3. CHECK IF PATTERN ARE MATCHING DIAGONALLY 
        else if (vec![&firstrow[2], &secondrow[1], &thirdrow[0]] == idk) || (vec![&firstrow[0], &secondrow[1], &thirdrow[2]] == idk) {
            winner = "You're the winner!";
            break;
        } else if (vec![&firstrow[2], &secondrow[1], &thirdrow[0]] == idk2) || (vec![&firstrow[0], &secondrow[1], &thirdrow[2]] == idk2) {

            winner = "You lost!";
            break;
        }

        // AI LOGIC NOW

        if player_turn {
            loop {
                let rannumbforrow = thread_rng().gen_range(1..4);
                let random_index_from_row = thread_rng().gen_range(0..3);
                    
                if rannumbforrow == 1 {
                    if &firstrow[random_index_from_row] == "||"{
                        let _ = mem::replace(&mut firstrow[random_index_from_row], player_character.to_owned());
                        break;
                    }
                }else if rannumbforrow == 2 {
                    if &secondrow[random_index_from_row] == "||"{
                        let _ = mem::replace(&mut secondrow[random_index_from_row], player_character.to_owned());
                        break;
                    }
                }else if rannumbforrow == 3 {
                    if &thirdrow[random_index_from_row] == "||"{
                        let _ = mem::replace(&mut thirdrow[random_index_from_row], player_character.to_owned());
                        break;
                    }
                }

            }
            player_turn = false;
        }else {
           loop {
                let rannumbforrow = thread_rng().gen_range(1..4);
                let random_index_from_row = thread_rng().gen_range(0..3);

                if rannumbforrow == 1 {
                    if &firstrow[random_index_from_row] == "||"{
                        let _ = mem::replace(&mut firstrow[random_index_from_row], opponent_character.to_owned());
                        break;
                    }
                }else if rannumbforrow == 2 {
                    if &secondrow[random_index_from_row] == "||"{
                        let _ = mem::replace(&mut secondrow[random_index_from_row], opponent_character.to_owned());
                        break;
                    }
                }else if rannumbforrow == 3 {
                    if &thirdrow[random_index_from_row] == "||"{
                        let _ = mem::replace(&mut thirdrow[random_index_from_row], opponent_character.to_owned());
                        break;
                    }
                }
            }
            player_turn = true;
        }

        // PRINT RESULTS
        println!("\n\n{} {} {}\n{} {} {}\n{} {} {}", &firstrow[0],&firstrow[1],&firstrow[2],&secondrow[0],&secondrow[1],&secondrow[2],&thirdrow[0],&thirdrow[1],&thirdrow[2]);
        thread::sleep(time::Duration::from_secs(2));
    }

    winner
}
// fn pause() {
//     let mut stdout = io::stdout();

//     print!(stdout, "Press any key to continue ").unwrap();
//     stdout.flush().unwrap();

//     let _ = io::stdin().read(&mut [0u8]).unwrap();
// }

