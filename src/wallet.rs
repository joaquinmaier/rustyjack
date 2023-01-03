use std::error::Error;
use std::collections::VecDeque;
use crate::errors::{ NotEnoughMoneyError, NonExistentBetError };

pub struct Wallet {
    pub money: f64,
        bets: Option<VecDeque<f64>>
}

impl Wallet
{
    pub fn new( initial_money: i32 ) -> Wallet {
        Wallet { money: initial_money as f64, bets: None }
    }

    pub fn bet( &mut self, amount: f64 ) -> Result<(), Box<NotEnoughMoneyError>>{
        if amount > self.money              { return Err( Box::new( NotEnoughMoneyError ) ); }

        self.money -= amount;

        if self.bets == None {
            let mut temp_vec = VecDeque::new();
            temp_vec.push_back( amount );

            self.bets = Some( temp_vec );

        } else {
            self.bets.as_mut().unwrap().push_back( amount );

        }

        Ok(())
    }

    pub fn give_win_reward( &mut self ) -> Result<(), Box<NonExistentBetError>> {
        if self.bets == None                { return Err( Box::new( NonExistentBetError ) ); }

        self.money += 2.0 * self.bets.as_mut().unwrap().pop_front().unwrap();

        Ok(())
    }

    pub fn take_bet( &mut self ) -> Result<(), Box<NonExistentBetError>> {
        if self.bets == None                { return Err( Box::new( NonExistentBetError ) ); }

        self.bets.as_mut().unwrap().pop_front();

        Ok(())
    }

    pub fn give_win_reward_bj( &mut self ) -> Result<(), Box<NonExistentBetError>> {
        if self.bets == None                { return Err( Box::new( NonExistentBetError ) ); }

        self.money += 2.5 * self.bets.as_mut().unwrap().pop_front().unwrap();

        Ok(())
    }

    pub fn push_bet( &mut self ) -> Result<(), Box<NonExistentBetError>> {
        if self.bets == None                { return Err( Box::new( NonExistentBetError ) ); }

        self.money += self.bets.as_mut().unwrap().pop_front().unwrap();

        Ok(())
    }

    pub fn double_bet( &mut self ) -> Result<(), Box<dyn Error>> {
        if self.bets == None                { return Err( Box::new( NonExistentBetError ) ); }

        let temp    = self.bets.as_mut().unwrap().pop_back().unwrap();

        if self.money - temp <= 0.      { return Err( Box::new( NotEnoughMoneyError ) ); }

        self.money  -= temp;
        self.bets.as_mut().unwrap().push_back( temp + temp );

        Ok(())
    }
}
