/*
Copyright (c) 2023 Joaquin Maier

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use colour::*;
use std::error::Error;
use std::collections::VecDeque;
use crate::errors::{ NotEnoughMoneyError, NonExistentBetError, InvalidStateError };

pub const PLAYER_OBJECTIVE: i32 = 100_000;

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

        if self.money - temp < 0. {
            self.bets.as_mut().unwrap().push_back( temp );
            return Err( Box::new( NotEnoughMoneyError ) );
        }

        self.money  -= temp;
        self.bets.as_mut().unwrap().push_back( temp + temp );

        Ok(())
    }

    pub fn print_info( &self ) {
        dark_grey_ln!( "Current bets: {:?}\nYour money: {}", self.bets.clone().unwrap(), self.money );
    }

    pub fn pay_insurance( &mut self ) -> Result<(), Box<dyn Error>> {
        if self.bets == None                { return Err( Box::new( NonExistentBetError ) ); }

        // Cannot split before prompting insurance
        if self.bets.as_ref().unwrap().len() > 1     { return Err( Box::new( InvalidStateError ) ); }

        let insurance_cost  = self.bets.as_ref().unwrap()[0] * 0.5;

        if self.money - insurance_cost < 0. { return Err( Box::new( NotEnoughMoneyError ) ); }

        self.money  -= insurance_cost;

        Ok(())
    }

    pub fn return_insurance( &mut self ) -> Result<(), Box<NonExistentBetError>> {
        if self.bets == None                { return Err( Box::new( NonExistentBetError ) ); }

        // Add the original bet + the cost of the insurance, so you lose nothing
        self.money += self.bets.as_ref().unwrap()[0] + self.bets.as_ref().unwrap()[0] * 0.5;

        Ok( () )
    }

    pub fn can_pay( &self, cost: f64 ) -> bool {
        return self.money >= cost;
    }

    pub fn has_won( &self ) -> bool {
        return self.money >= PLAYER_OBJECTIVE as f64;
    }
}

