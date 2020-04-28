use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io::{self, Read};

#[derive(Clone, Debug)]
enum Suit {
    Hearts,
    Diamond,
    Spades,
    Clubs
}

#[derive(Clone, Debug)]
enum Face {
    Number(u8),
    Face(&'static str)
}

#[derive(Clone, Debug)]
struct Card {
    value: Face,
    suit: Suit
}

#[derive(Default)]
struct Deck(Vec<Card>);

impl Deck {
    fn new() -> Self {
        let mut deck = Deck::default();
        (2..=10u8).for_each(|x| {
            deck.0.push(Card { value : Face::Number(x), suit: Suit::Hearts });
            deck.0.push(Card { value : Face::Number(x), suit: Suit::Diamond });
            deck.0.push(Card { value : Face::Number(x), suit: Suit::Spades });
            deck.0.push(Card { value : Face::Number(x), suit: Suit::Clubs });

        });

        ["J", "Q", "K", "A"].iter().for_each(|x| {
        deck.0.push(Card { value : Face::Face(x), suit: Suit::Hearts });
        deck.0.push(Card { value : Face::Face(x), suit: Suit::Diamond });
        deck.0.push(Card { value : Face::Face(x), suit: Suit::Spades });
        deck.0.push(Card { value : Face::Face(x), suit: Suit::Clubs });
        });

        let mut rng = rand::thread_rng();
        deck.0.shuffle(&mut rng);

        deck

    }

    fn draw_1(&mut self) -> Option<Card> {
        self.0.pop()
    }

    fn draw_2(&mut self) -> Vec<Card> {
        let cards = self.0.iter().take(2).cloned().collect::<Vec<Card>>();
        self.0 = self.0[2..].to_vec();
        cards

    }
}

#[derive(Debug)]
enum State {
    Winner,
    Bust(u8),
    Under(u8)
}

#[derive(Debug, Default)]
struct Hand( Vec<Card>);

impl Hand {

    pub fn first_deal(&mut self, deck: &mut Deck) {
        let cards = deck.draw_2();

        self.0.extend_from_slice(&cards[..]);
    }

    pub fn hit(&mut self, deck: &mut Deck) {
        let card = deck.draw_1().unwrap();
        self.0.push(card);
    }

    pub fn evaluate(&self) -> State {
        let  mut flag = false;
        let mut value = self.0.iter().fold(0u8, |acc, card| {
            match card.value {
                Face::Number(x) => acc + x,
                Face::Face(x)  if x == "A"  => {
                    flag = true;
                    acc + 11
                },
                Face::Face(_) => acc + 10,
            }});

        if flag && value > 21  {
            value -= 10;
        }


        match value {
            21 => State::Winner,
            x if x > 21 => State::Bust(x),
            x => State::Under(x)
        }

    }

}

#[derive(Debug, Default)]
struct Player {
    hand: Hand,
}

#[derive(Debug, Default)]
struct Dealer {
    hand: Hand,
}

fn main() {
    let mut deck = Deck::new();
    let card = deck.draw_2();

    let mut player = Player::default();
    let mut dealer = Dealer::default();

    player.hand.first_deal(&mut deck);
    dealer.hand.first_deal(&mut deck);

    println!("Your cards:");
    player.hand.0.iter().for_each(|x| {
        println!("{:?} ", x);
    });

    loop {
        match player.hand.evaluate() {
            State::Winner => { println!("you win"); break; }
            State::Bust(_) => { println!("you lose"); break; },
            State::Under(player_v) => {
                println!("Hit or fold? 1. Hit 2. Fold");

                let stdin = io::stdin();
                let mut buffer = [0; 1];
                let mut handle = stdin.lock();
                handle.read_exact(&mut buffer);
                let choice: Option<()> = if buffer[0] == b'1' { Some(()) } else { None };

                match choice {
                    Some(_) => {
                        player.hand.hit(&mut deck);
                        player.hand.0.iter().for_each(|x| {
                            println!("{:?} ", x);
                        });
                        continue;
                    },
                    _ => {
                    }
                };

                loop {
                    println!("Dealer cards:");
                    dealer.hand.0.iter().for_each(|x| {
                        println!("{:?} ", x);
                    });

                    dealer.hand.hit(&mut deck);

                    match dealer.hand.evaluate() {
                        State::Winner => { println!("dealer wins"); break; }
                        State::Bust(_) => { println!("dealer loses"); break; }
                        State::Under(x) if x >= 17 =>  {
                            if player_v > x {
                                println!("you win");
                            } else {
                                println!("you lose");
                            }
                        },
                        _ => {

                            continue;
                        }
                    }
                }
                break;
            }
        }
    }



}