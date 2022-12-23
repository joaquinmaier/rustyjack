use crate::hand::Hand;
use crate::card::components::SumType;

pub fn determine_winner( hands: &Vec<Hand> ) -> usize {
    let mut temp_winner: (usize, i32)  = ( 0, 0 );
    for (index, hand) in hands.iter().enumerate() {
        match hand.sum_value.clone().unwrap() {
            SumType::SingleValue( n ) => {
                if n <= 21 && n > temp_winner.1 {
                    temp_winner = ( index, n );

                } else if n == temp_winner.1 {
                    // * This is a band-aid. Will not work properly with more than a single player (pt 1)
                    return 255;
                }
            },
            SumType::MultipleValue( n1, n2 ) => {
                if n2 <= 21 && n2 > temp_winner.1 {
                    temp_winner = ( index, n2 );

                }
                else if n1 <= 21 && n1 > temp_winner.1 {
                    temp_winner = ( index, n1 );

                } else if n1 == temp_winner.1 || n2 == temp_winner.1 {
                    // * This is a band-aid. Will not work properly with more than a single player (pt 2)
                    return 255;

                }
            }
        }
    }

    return temp_winner.0;
}
