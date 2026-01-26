use rand::seq::SliceRandom;
use std::io;

fn calculate_hand_value(hand: &Vec<i32>) -> i32 {
    let mut total: i32 = hand.iter().sum();
    let ace_count = hand.iter().filter(|&&card| card == 11).count();

    let mut aces_to_convert = ace_count;
    while total > 21 && aces_to_convert > 0 {
        total -= 10;
        aces_to_convert -= 1;
    }

    total
}

fn main() {
    let mut decision = String::new();
    let mut deck: Vec<i32> = (1..=11).collect();
    deck.shuffle(&mut rand::thread_rng());

    let first_card = deck.pop().unwrap();
    let mut player_hand: Vec<i32> = vec![first_card];
    let mut dealer_hand: Vec<i32> = vec![];
    dealer_hand.push(deck.pop().unwrap());
    dealer_hand.push(deck.pop().unwrap());
    println!("Your first card: {}", first_card);

    while decision != "s" {
        println!("Do you want to (h)it or (s)tand?");
        decision.clear();
        io::stdin()
            .read_line(&mut decision)
            .expect("Failed to read line");
        decision = decision.trim().to_lowercase();
        if decision == "h" {
            let new_card = deck.pop().unwrap();
            player_hand.push(new_card);
            println!("You drew a {}", new_card);
            let player_total = calculate_hand_value(&player_hand);
            println!("Your total is now {}", player_total);
            if player_total > 21 {
                println!("You busted! Dealer wins.");
                return;
            }
            if player_total == 21 {
                println!("You hit 21! You win!");
                return;
            }
        } else if decision == "s" {
            let player_total = calculate_hand_value(&player_hand);
            let mut dealer_total = calculate_hand_value(&dealer_hand);
            println!("Dealer's cards: {:?}", dealer_hand);
            while dealer_total < 17 {
                let new_card = deck.pop().unwrap();
                dealer_hand.push(new_card);
                dealer_total = calculate_hand_value(&dealer_hand);
                println!("Dealer draws a {}", new_card);
            }
            println!("Dealer's total is {}", dealer_total);
            if dealer_total > 21 || player_total > dealer_total {
                println!("You win!");
            } else if player_total < dealer_total {
                println!("Dealer wins!");
            } else {
                println!("It's a tie!");
            }
            return;
        } else {
            println!("Invalid input, please enter 'h' or 's'.");
        }
    }

    println!("The game is over");
}
