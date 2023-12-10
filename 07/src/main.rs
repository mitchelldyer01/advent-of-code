use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part one: {:#?}", part_one()?);
    println!("Part two: {:#?}", part_two()?);
    Ok(())
}

fn part_one() -> Result<i64, Box<dyn Error>> {
    let mut sum = 0;

    let mut game = parse_input_into_game("input.txt")?;

    game.camels.iter_mut().for_each(|camel| {
        camel.get_hand_from_cards();
    });

    let ranked = game.rank_camels();

    for i in 0..ranked.len() {
        sum += ranked[i].bid * (i as i64 + 1);
    }

    Ok(sum)
}

fn part_two() -> Result<i64, Box<dyn Error>> {
    let mut sum = 0;

    let mut game = parse_input_into_game("input.txt")?;

    game.camels.iter_mut().for_each(|camel| {
        camel.get_hand_from_cards_with_jokers();
    });

    let ranked = game.rank_camels_with_jokers();

    for i in 0..ranked.len() {
        sum += ranked[i].bid * (i as i64 + 1);
    }

    Ok(sum)
}
fn parse_input_into_game(file_path: &str) -> Result<Game, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut game = Game{ camels: vec![] };

    for line in reader.lines() {
        let text = line?; 
        let mut card_and_bet = text.split_whitespace();
        game.camels.push(Camel{ hand: Hand::None, cards: card_and_bet.next().unwrap().to_string(), bid: card_and_bet.next().unwrap().parse::<i64>()? });
    }

    Ok(game)
}

#[derive(Debug, PartialEq, Clone)]
enum Hand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    None
}


#[derive(Debug)]
struct Game {
    camels: Vec<Camel>
}

impl Game {
    fn rank_camels(&mut self) -> Vec<Camel> {
        let mut ranked: Vec<Camel> = vec![];

        // break them down by Hand
        let mut nones: Vec<Camel> = self.camels.iter().filter(|camel| camel.hand == Hand::None).cloned().collect();
        let mut onepairs: Vec<Camel> = self.camels.iter().filter(|camel| camel.hand == Hand::OnePair).cloned().collect();
        let mut twopairs: Vec<Camel> = self.camels.iter().filter(|camel| camel.hand == Hand::TwoPair).cloned().collect();
        let mut threes: Vec<Camel> = self.camels.iter().filter(|camel| camel.hand == Hand::ThreeOfAKind).cloned().collect();
        let mut fullhouses: Vec<Camel> = self.camels.iter().filter(|camel| camel.hand == Hand::FullHouse).cloned().collect();
        let mut fours: Vec<Camel> = self.camels.iter().filter(|camel| camel.hand == Hand::FourOfAKind).cloned().collect();
        let mut fives: Vec<Camel> = self.camels.iter().filter(|camel| camel.hand == Hand::FiveOfAKind).cloned().collect();

        nones.sort_by(compare_camels);
        onepairs.sort_by(compare_camels);
        twopairs.sort_by(compare_camels);
        threes.sort_by(compare_camels);
        fullhouses.sort_by(compare_camels);
        fours.sort_by(compare_camels);
        fives.sort_by(compare_camels);

        ranked.append(&mut nones);
        ranked.append(&mut onepairs);
        ranked.append(&mut twopairs);
        ranked.append(&mut threes);
        ranked.append(&mut fullhouses);
        ranked.append(&mut fours);
        ranked.append(&mut fives);

        ranked
    }

    fn rank_camels_with_jokers(&mut self) -> Vec<Camel> {
        let mut ranked: Vec<Camel> = vec![];

        // break them down by Hand
        let mut nones: Vec<Camel> = self.camels.iter().filter(|camel| camel.hand == Hand::None).cloned().collect();
        let mut onepairs: Vec<Camel> = self.camels.iter().filter(|camel| camel.hand == Hand::OnePair).cloned().collect();
        let mut twopairs: Vec<Camel> = self.camels.iter().filter(|camel| camel.hand == Hand::TwoPair).cloned().collect();
        let mut threes: Vec<Camel> = self.camels.iter().filter(|camel| camel.hand == Hand::ThreeOfAKind).cloned().collect();
        let mut fullhouses: Vec<Camel> = self.camels.iter().filter(|camel| camel.hand == Hand::FullHouse).cloned().collect();
        let mut fours: Vec<Camel> = self.camels.iter().filter(|camel| camel.hand == Hand::FourOfAKind).cloned().collect();
        let mut fives: Vec<Camel> = self.camels.iter().filter(|camel| camel.hand == Hand::FiveOfAKind).cloned().collect();

        nones.sort_by(compare_camels_with_jokers);
        onepairs.sort_by(compare_camels_with_jokers);
        twopairs.sort_by(compare_camels_with_jokers);
        threes.sort_by(compare_camels_with_jokers);
        fullhouses.sort_by(compare_camels_with_jokers);
        fours.sort_by(compare_camels_with_jokers);
        fives.sort_by(compare_camels_with_jokers);

        ranked.append(&mut nones);
        ranked.append(&mut onepairs);
        ranked.append(&mut twopairs);
        ranked.append(&mut threes);
        ranked.append(&mut fullhouses);
        ranked.append(&mut fours);
        ranked.append(&mut fives);

        ranked
    }
}

#[derive(Debug, Clone)]
struct Camel {
    hand: Hand,
    cards: String,
    bid: i64,
}

impl Camel {
    fn get_hand_from_cards_with_jokers(&mut self) {
        let old_hand = &self.cards;
        let hand: String = swap_jokers_to_end_of_hand(old_hand.to_string()).into_iter().collect();

        if hand.chars().all(|card| card == hand.chars().nth(0).unwrap() || card == 'J') {
            self.hand = Hand::FiveOfAKind;
            return
        }

        if hand.chars().filter(|&card| card == hand.chars().nth(0).unwrap() || card == 'J').count() == 4 || 
           hand.chars().filter(|&card| card == hand.chars().nth(1).unwrap() || card == 'J').count() == 4 {
            self.hand = Hand::FourOfAKind;
            return
        }

        for i in 0..4 {
            if i < 3 {
                // check for three of a kind
                let (matchers, leftovers): (Vec<_>, Vec<_>) = hand.chars().partition(|&x| x == hand.chars().nth(i).unwrap() || x == 'J');
                if matchers.len() == 3 && leftovers.iter().all(|&x| x == leftovers[0]) {
                    self.hand = Hand::FullHouse;
                    return
                }

                if  matchers.len() == 3 {
                    self.hand = Hand::ThreeOfAKind;
                    return
                }
            }

            if i < 4 {
                let (matchers, leftovers): (Vec<_>, Vec<_>) = hand.chars().partition(|&x| x == hand.chars().nth(i).unwrap() || x == 'J');
                // check for full house
                if matchers.len() == 2 && leftovers.iter().all(|&x| x == leftovers[0]) {
                    self.hand = Hand::FullHouse;
                    return
                }
                // check for two pair
                if matchers.len() == 2 && (leftovers[0] == leftovers[1] || leftovers[0] == leftovers[2] || leftovers[1] == leftovers[2]) {
                    self.hand = Hand::TwoPair;
                }

                // check for one pair
                if matchers.len() == 2 && (leftovers[0] != leftovers[1] && leftovers[0] != leftovers[2] && leftovers[1] != leftovers[2]) {
                    self.hand = Hand::OnePair;
                }
            }
        }
    }

    fn get_hand_from_cards(&mut self) {
        if self.cards.chars().all(|card| card == self.cards.chars().nth(0).unwrap()) {
            self.hand = Hand::FiveOfAKind;
            return
        }

        for i in 0..4 {
            if i < 2 {
                // check for four of a kind
                if self.cards.chars().filter(|&card| card == self.cards.chars().nth(i).unwrap()).count() == 4 {
                    self.hand = Hand::FourOfAKind;
                    return
                } 
            }

            if i < 3 {
                // check for three of a kind
                if self.cards.chars().filter(|&card| card == self.cards.chars().nth(i).unwrap()).count() == 3 {
                    match self.hand {
                        Hand::OnePair | Hand::TwoPair => self.hand = Hand::FullHouse,
                        Hand::FullHouse => self.hand = Hand::FullHouse,
                        _ => self.hand = Hand::ThreeOfAKind,
                    }
                }
            }

            if i < 4 {
                // check for two pair
                // check for one pair
                // check for full house
                let (matchers, leftovers): (Vec<_>, Vec<_>) = self.cards.chars().partition(|&x| x == self.cards.chars().nth(i).unwrap());
                if matchers.len() == 2 {
                    match self.hand {
                        Hand::ThreeOfAKind => self.hand = Hand::FullHouse,
                        Hand::OnePair => {
                            if leftovers[0] == leftovers[1] || leftovers[1] == leftovers[2] || leftovers[0] == leftovers[2] {
                                self.hand = Hand::TwoPair;
                            }
                        }, 
                        Hand::None => self.hand = Hand::OnePair,
                        _ => {},
                    }
                }
            }
        }
    }
}

fn swap_jokers_to_end_of_hand(hand: String) -> Vec<char> {
    let mut new_hand: Vec<char> = vec![]; 

    let (jokers, leftovers): (Vec<_>, Vec<_>) = hand.chars().partition(|&x| x == 'J');

    new_hand.extend(leftovers.iter());
    new_hand.extend(jokers.iter());

    new_hand
}

fn get_card_weight(c: char) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    }
}

fn get_card_weight_with_jokers(c: char) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => 0,
    }

}

fn compare_camels_with_jokers(hand1: &Camel, hand2: &Camel) -> std::cmp::Ordering {
    hand1.cards.chars().zip(hand2.cards.chars()).fold(
        std::cmp::Ordering::Equal,
        |acc, (card1, card2)| {
            let order1 = get_card_weight_with_jokers(card1);
            let order2 = get_card_weight_with_jokers(card2);
            acc.then(order1.cmp(&order2))
        },
    )
}

fn compare_camels(hand1: &Camel, hand2: &Camel) -> std::cmp::Ordering {
    hand1.cards.chars().zip(hand2.cards.chars()).fold(
        std::cmp::Ordering::Equal,
        |acc, (card1, card2)| {
            let order1 = get_card_weight(card1);
            let order2 = get_card_weight(card2);
            acc.then(order1.cmp(&order2))
        },
    )
}

#[test]
fn test_get_hand_from_cards_with_joker() {
    let mut first = Camel { hand: Hand::None, cards: "32T3K".to_string(), bid: 765 };
    let mut second = Camel { hand: Hand::None, cards: "AAT6J".to_string(), bid: 765 };
    let mut third = Camel { hand: Hand::None, cards: "KK677".to_string(), bid: 28 };
    let mut fourth = Camel { hand: Hand::None, cards: "AQJA6".to_string(), bid: 28 };
    let mut sixth = Camel { hand: Hand::None, cards: "QQQQA".to_string(), bid: 483 };
    let mut seventh = Camel { hand: Hand::None, cards: "QQQQQ".to_string(), bid: 483 };
    let mut eigth = Camel { hand: Hand::None, cards: "Q1234".to_string(), bid: 483 };
    let mut ninth = Camel { hand: Hand::None, cards: "QQKKK".to_string(), bid: 483 };
    let mut tenth = Camel { hand: Hand::None, cards: "QKKKQ".to_string(), bid: 483 };
    let mut eleventh = Camel { hand: Hand::None, cards: "JJKKK".to_string(), bid: 483 };
    let mut twelfth = Camel { hand: Hand::None, cards: "334J4".to_string(), bid: 483 };

    first.get_hand_from_cards_with_jokers();
    second.get_hand_from_cards_with_jokers();
    third.get_hand_from_cards_with_jokers();
    fourth.get_hand_from_cards_with_jokers();
    sixth.get_hand_from_cards_with_jokers();
    seventh.get_hand_from_cards_with_jokers();
    eigth.get_hand_from_cards_with_jokers();
    ninth.get_hand_from_cards_with_jokers();
    tenth.get_hand_from_cards_with_jokers();
    eleventh.get_hand_from_cards_with_jokers();
    twelfth.get_hand_from_cards_with_jokers();

    assert_eq!(first.hand, Hand::OnePair);
    assert_eq!(second.hand, Hand::ThreeOfAKind);
    assert_eq!(third.hand, Hand::TwoPair);
    assert_eq!(fourth.hand, Hand::ThreeOfAKind);
    assert_eq!(sixth.hand, Hand::FourOfAKind);
    assert_eq!(seventh.hand, Hand::FiveOfAKind);
    assert_eq!(eigth.hand, Hand::None);
    assert_eq!(ninth.hand, Hand::FullHouse);
    assert_eq!(tenth.hand, Hand::FullHouse);
    assert_eq!(eleventh.hand, Hand::FiveOfAKind);
    assert_eq!(twelfth.hand, Hand::FullHouse);
}

#[test]
fn test_get_hand_from_cards() {
    let mut first = Camel { hand: Hand::None, cards: "32T3K".to_string(), bid: 765 };
    let mut second = Camel { hand: Hand::None, cards: "T55J5".to_string(), bid: 684 };
    let mut third = Camel { hand: Hand::None, cards: "KK677".to_string(), bid: 28 };
    let mut fourth = Camel { hand: Hand::None, cards: "KTJJT".to_string(), bid: 220 };
    let mut fifth = Camel { hand: Hand::None, cards: "QQQJA".to_string(), bid: 483 };
    let mut sixth = Camel { hand: Hand::None, cards: "QQQQA".to_string(), bid: 483 };
    let mut seventh = Camel { hand: Hand::None, cards: "QQQQQ".to_string(), bid: 483 };
    let mut eigth = Camel { hand: Hand::None, cards: "Q1234".to_string(), bid: 483 };
    let mut ninth = Camel { hand: Hand::None, cards: "QQKKK".to_string(), bid: 483 };
    let mut tenth = Camel { hand: Hand::None, cards: "QKKKQ".to_string(), bid: 483 };

    first.get_hand_from_cards();
    second.get_hand_from_cards();
    third.get_hand_from_cards();
    fourth.get_hand_from_cards();
    fifth.get_hand_from_cards();
    sixth.get_hand_from_cards();
    seventh.get_hand_from_cards();
    eigth.get_hand_from_cards();
    ninth.get_hand_from_cards();
    tenth.get_hand_from_cards();

    assert_eq!(first.hand, Hand::OnePair);
    assert_eq!(second.hand, Hand::ThreeOfAKind);
    assert_eq!(third.hand, Hand::TwoPair);
    assert_eq!(fourth.hand, Hand::TwoPair);
    assert_eq!(fifth.hand, Hand::ThreeOfAKind);
    assert_eq!(sixth.hand, Hand::FourOfAKind);
    assert_eq!(seventh.hand, Hand::FiveOfAKind);
    assert_eq!(eigth.hand, Hand::None);
    assert_eq!(ninth.hand, Hand::FullHouse);
    assert_eq!(tenth.hand, Hand::FullHouse);
}

#[test]
fn test_compare_camels() {
    let mut camels = vec![
        Camel { hand: Hand::None, cards: "QQJJO".to_string(), bid: 483 },
        Camel { hand: Hand::None, cards: "QJJQO".to_string(), bid: 483 },
        Camel { hand: Hand::None, cards: "JQJQO".to_string(), bid: 483 },
    ];

    camels.sort_by(compare_camels);

    assert_eq!(camels[0].cards, "JQJQO");
    assert_eq!(camels[1].cards, "QJJQO");
    assert_eq!(camels[2].cards, "QQJJO");
}
