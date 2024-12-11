use std::{cmp::{self, Ordering}, f32::consts::TAU, fs, iter};

fn symbol_value_jokers(c: char) -> Option<u32>{
    if let Some(x) = c.to_digit(10) { return Some(x); };
    match c {
        'T' => Some(10),
        'J' => Some(11),
        'Q' => Some(12),
        'K' => Some(13),
        'A' => Some(14),
        _ => None,
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandType {
    OfaKind{n: usize, card: u32},
    TwoPairs{pair1: u32, pair2: u32},
    Full{pair:u32, triple: u32}
}
impl HandType{
    // fn same_type(&self, other: &Hand) -> bool{

    // }
    fn strenght(&self)-> usize {
        match self {
            HandType::OfaKind {n, ..} => {
                match n {
                    &x if x<=2 => x,
                    3 => 4,
                    &x if x>3 => x+2,
                    _ => 0
                }
            },
            HandType::TwoPairs {..} => 3,
            HandType::Full {..} => 5,
        }
    }
}

// impl PartialOrd for Hand {
//     fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
//         let (s1,s2) = (self.strenght(), other.strenght());
//         Some(
//         match (self, other) {
//             (Hand::OfaKind { n: n1, card: s }, Hand::OfaKind { n: n2, card: o } ) if n1==n2 => s.cmp(o),
//             (Hand::TwoPairs { pair1: s1, pair2: s2 }, Hand::TwoPairs { pair1: o1, pair2: o2 }) => todo!(),
//             (Hand::Full { pair: s2, triple: s3 }, Hand::Full { pair: o2, triple: o3 }) => todo!(),
//             _ => { self.strenght().cmp(&other.strenght()) },
//         })
        
//     }
// }

// impl Ord for Hand {
//     // fn cmp(&self, other: &Self) -> cmp::Ordering {
//     //     let (s1,s2) = (self.strenght(), other.strenght());
//     //     match (self, other) {
//     //         (Hand::OfaKind { n: n1, card: s }, Hand::OfaKind { n: n2, card: o } ) if n1==n2 => s.cmp(o),
//     //         (Hand::TwoPairs { pair1: s1, pair2: s2 }, Hand::TwoPairs { pair1: o1, pair2: o2 }) => todo!(),
//     //         (Hand::Full { pair: s2, triple: s3 }, Hand::Full { pair: o2, triple: o3 }) => todo!(),
//     //         _ => { self.strenght().cmp(&other.strenght()) },
//     //     }
//     // }
//     fn cmp(&self, other: &Self) -> cmp::Ordering {
//         self.partial_cmp(other).unwrap();
//     }
// }
fn read_hand(hand: &[u32]) -> HandType {
    println!("reading {hand:?}");
    let mut count = [1;5];
    for i in 0..5{
        for j in  i+1..5{
            if hand[i] == hand[j]{
                count[i] += 1;
            }
        }
    }
    let mut first = None;
    println!("\tcount = {count:?}");
    for (i,n) in count.into_iter().enumerate(){
        let card = hand[i];
        if n>1 {
            match first {
                Some((c, m)) if card != c => {
                    if n == 3 { return HandType::Full { pair: c, triple: card }; }
                    else if m == 3 { return HandType::Full { pair: card, triple: c }; }
                    else {return HandType::TwoPairs { pair1: card, pair2: c }}
                },
                None => { first =  Some((card, n)) },
                _ => {}
            }
        }
    }
    match first {
        Some((card, n)) => HandType::OfaKind { n, card },
        None => HandType::OfaKind { n: 1, card: *hand.iter().max().unwrap() },
    }
    
}
fn read_hand_jokers(hand: &[u32]) -> HandType {
    println!("reading {hand:?}");
    let mut count = [1;5];
    for i in 0..5{
        for j in  i+1..5{
            if hand[i] == hand[j]{
                count[i] += 1;
            }
        }
    }
    let n_jokers = hand.iter().filter(|x| **x==1).count();
    if let Some((max_i, _max_c)) = count.iter().enumerate().filter(|x| hand[x.0] != 1).max_by_key(|x| *x.1){
        count[max_i] += n_jokers;
    }
    let mut first = None;
    println!("\tcount = {count:?}");
    for (i,n) in count.into_iter().enumerate(){
        let card = hand[i];
        if n>1 && card!=1 {
            match first {
                Some((c, m)) if card != c => {
                    if n == 3 { return HandType::Full { pair: c, triple: card }; }
                    else if m == 3 { return HandType::Full { pair: card, triple: c }; }
                    else {return HandType::TwoPairs { pair1: card, pair2: c }}
                },
                None => { first =  Some((card, n)) },
                _ => {}
            }
        }
    }
    match first {
        Some((card, n)) => HandType::OfaKind { n, card },
        None => HandType::OfaKind { n: if n_jokers==0 {1} else {5}, card: *hand.iter().max().unwrap() },
    }
    
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand{
    type_: HandType,
    cards: Vec<u32>,
    bid: u32,
}
impl PartialOrd for Hand{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match self.type_.strenght().cmp(&other.type_.strenght()) {
            cmp::Ordering::Equal => {
                for (xs, xo) in iter::zip(self.cards.iter(), other.cards.iter()){
                    let compared = xs.cmp(xo);
                    if compared!=Ordering::Equal {return Some(compared);}
                }
                Some(cmp::Ordering::Equal)
            },
            ord => Some(ord),
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn jokerify(hand: &mut Hand){
    for x in hand.cards.iter_mut(){
        if *x==11 { *x=1 };
    }
    let n_jokers = hand.cards.iter().filter(|x| **x==1).count();
    if n_jokers>0{
        hand.type_ = match hand.type_ {
            HandType::OfaKind { n, card } => { 
                if card==11 { HandType::OfaKind { n: usize::min(5, n_jokers+1), card: 1 } }
                else { HandType::OfaKind { n: n+n_jokers, card } }
            },
            HandType::TwoPairs { pair1, pair2 } => {
                if pair1==11 { HandType::OfaKind { n: 4, card: pair2 } }
                else if pair2==11 { HandType::OfaKind { n: 4, card: pair1 } }
                else { HandType::Full { pair: pair1, triple: pair2 } }
            },
            HandType::Full { pair, triple } => {
                if pair==11 { HandType::OfaKind { n: 5, card: triple } }
                else if triple==11 { HandType::OfaKind { n: 5, card: pair } }
                else { HandType::Full { pair, triple } }
            }
        }
    }
}

fn main() {
    let mut sol = (0,0);
    println!("{:?}",symbol_value_jokers('9'));
    let file = "input";
    // let file = "input-test";
    // let file = "test";
    let data = fs::read_to_string(file).expect("and by unexpected I mean: COMPLETELY EXPECTED!");
    let mut hands = Vec::<Hand>::new();
    let mut  joker_hands = Vec::<Hand>::new();
    // let mut bids = Vec::<u32>::new();
    for line in data.lines(){
        let (hand, bid) = line.split_once(' ').unwrap();
        let bid: u32 = bid.parse().unwrap();
        let hand_cards = hand.chars().map(|c| symbol_value_jokers(c).unwrap()).collect::<Vec<_>>();
        // let hand = read_hand_jokers(&hand_cards);
        let hand = read_hand(&hand_cards);
        let mut hand = Hand { type_: hand, cards: hand_cards, bid };
        hands.push(hand.clone());
        jokerify(&mut hand);
        joker_hands.push(hand);
        // bids.push(bid);
    }
    hands.sort();
    for (i,hand) in hands.iter().enumerate(){
        let i = i+1;
        println!("Rank {i}:  {:?}", hand);
        sol.0 += i as u32 * hand.bid;
    }
    joker_hands.sort();
    for (i,hand) in joker_hands.iter().enumerate(){
        let i = i+1;
        println!("Rank {i}:  {:?}", hand);
        sol.1 += i as u32 * hand.bid;
    }
    println!("{:?}", sol)
}
