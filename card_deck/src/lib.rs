use rand::Rng;
#[derive(PartialEq,Debug,Clone)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club
}

impl Suit {
	pub fn random() -> Suit {
        let rng = rand::thread_rng().gen_range(1,4);
        Suit::translate(rng)
	}
    
	pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club
        }
	}
}
#[derive(PartialEq, Debug,Clone)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8)
}
impl Rank {
	pub fn random() -> Rank {
        let value: u8 = rand::thread_rng().gen_range(1,14);
        Rank::translate(value)
	}
    
	pub fn translate(value: u8) -> Rank {
            match value {
                1 => Rank::Ace, 
                2 => Rank::King, 
                3 => Rank::Queen, 
                4 => Rank::Jack, 
                5 => Rank::Number(2), 
                6 => Rank::Number(3), 
                7 => Rank::Number(4),
                8 => Rank::Number(5), 
                9 => Rank::Number(6), 
                10 => Rank::Number(7), 
                11 => Rank::Number(8), 
                12 => Rank::Number(9), 
                _ => Rank::Number(10)
            }
	}
}
#[derive(Debug)]
pub struct Card {
	pub suit: Suit,
	pub rank: Rank,
}
pub fn winner_card(card: &Card) -> bool {
    let winning_card = Card {suit:Suit::Spade, rank:Rank::Ace};
    if card.suit == winning_card.suit {
        if card.rank == winning_card.rank {
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let your_card = Card {
		rank: Rank::random(),
		suit: Suit::random(),
	};

	println!("Your card is {:?}", your_card);

	// Now if the card is an Ace of Spades print "You are the winner"
	if winner_card(&your_card) {
		println!("You are the winner!");
	}
    }
}
