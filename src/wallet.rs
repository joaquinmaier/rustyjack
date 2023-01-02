use std::error::Error;
use crate::errors::{ NotEnoughMoneyError, NonExistentBetError };

pub struct Wallet {
    pub money: f64,
        bet: Option<f64>
}

impl Wallet
{
    pub fn new( initial_money: i32 ) -> Wallet {
        Wallet { money: initial_money as f64, bet: None }
    }

    pub fn bet( &mut self, amount: f64 ) -> Result<(), Box<NotEnoughMoneyError>>{
        if amount > self.money { return Err( Box::new( NotEnoughMoneyError ) ); }

        self.money -= amount;
        self.bet = Some(amount);

        Ok(())
    }

    pub fn give_win_reward( &mut self ) -> Result<(), Box<NonExistentBetError>> {
        if self.bet == None              { return Err( Box::new( NonExistentBetError ) ); }

        self.money += 2.0 * self.bet.unwrap();
        self.bet = None;

        Ok(())
    }

    pub fn take_bet( &mut self ) -> Result<(), Box<NonExistentBetError>> {
        if self.bet == None              { return Err( Box::new( NonExistentBetError ) ); }

        self.bet = None;

        Ok(())
    }

    pub fn give_win_reward_bj( &mut self ) -> Result<(), Box<NonExistentBetError>> {
        if self.bet == None              { return Err( Box::new( NonExistentBetError ) ); }

        self.money += 2.5 * self.bet.unwrap();
        self.bet = None;

        Ok(())
    }

    pub fn push_bet( &mut self ) -> Result<(), Box<NonExistentBetError>> {
        if self.bet == None             { return Err( Box::new( NonExistentBetError ) ); }

        self.money += self.bet.unwrap();
        self.bet = None;

        Ok(())
    }

    pub fn double_bet( &mut self ) -> Result<(), Box<dyn Error>> {
        if self.bet == None             { return Err( Box::new( NonExistentBetError ) ); }

        let temp    = self.bet.unwrap();

        if self.money - temp <= 0.      { return Err( Box::new( NotEnoughMoneyError ) ); }

        self.money  -= temp;
        self.bet    = Some( temp + temp );

        Ok(())
    }

}
