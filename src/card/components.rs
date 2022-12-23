#[derive(std::cmp::PartialEq, Debug, Clone)]
pub enum CardType
{
    HEARTS = 0,
    CLUBS = 1,
    SPADES = 2,
    DIAMONDS = 3
}

#[derive(std::cmp::PartialEq, Debug, Clone)]
pub enum CardValue
{
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    TEN = 10,
    JACK = 11,
    QUEEN = 12,
    KING = 13,
    ACE = 1
}

impl CardValue
{
    pub fn from( value: usize ) -> CardValue {
        match value {
            0 => CardValue::TWO,
            1 => CardValue::THREE,
            2 => CardValue::FOUR,
            3 => CardValue::FIVE,
            4 => CardValue::SIX,
            5 => CardValue::SEVEN,
            6 => CardValue::EIGHT,
            7 => CardValue::NINE,
            8 => CardValue::TEN,
            9 => CardValue::JACK,
            10 => CardValue::QUEEN,
            11 => CardValue::KING,
            12 => CardValue::ACE,
            _ => { panic!( "Forbidden card value received" ); }
        }
    }

    pub fn to_int( value: &CardValue ) -> i32 {
        return match *value {
            CardValue::TWO => 2,
            CardValue::THREE => 3,
            CardValue::FOUR => 4,
            CardValue::FIVE => 5,
            CardValue::SIX => 6,
            CardValue::SEVEN => 7,
            CardValue::EIGHT => 8,
            CardValue::NINE => 9,
            CardValue::TEN => 10,
            CardValue::JACK => 10,
            CardValue::QUEEN => 10,
            CardValue::KING => 10,
            CardValue::ACE => 1
        };
    }

    pub fn to_str( value: &CardValue ) -> &'static str {
        return match *value {
            CardValue::TWO => "2",
            CardValue::THREE => "3",
            CardValue::FOUR => "4",
            CardValue::FIVE => "5",
            CardValue::SIX => "6",
            CardValue::SEVEN => "7",
            CardValue::EIGHT => "8",
            CardValue::NINE => "9",
            CardValue::TEN => "10",
            CardValue::JACK => "J",
            CardValue::QUEEN => "Q",
            CardValue::KING => "K",
            CardValue::ACE => "A"
        };
    }
}

#[derive(Debug, Clone)]
pub enum SumType
{
    SingleValue( i32 ),
    MultipleValue( i32, i32 ),
}

