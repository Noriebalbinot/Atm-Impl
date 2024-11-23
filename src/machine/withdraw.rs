use std::error::Error;

use crate::models::{bank::Bank, machine_model::AtmMachine};

impl AtmMachine {
    pub fn withdraw(&mut self, value: u32) -> Result<Bank, Box<dyn Error>> {
        let mut rest = value;
        let mut resultbank = Bank::default();
        if rest >= 100 {
            let mut billqtd = rest / 100;
            if billqtd > self.bank.onehundred {
                rest -= self.bank.onehundred * 100;
                resultbank.onehundred = self.bank.onehundred;
                self.bank.onehundred = 0
            } else {
                rest -= billqtd * 100;
                resultbank.onehundred = billqtd;
                self.bank.onehundred -= billqtd;
            }
        }
        if rest >= 50 {
            let mut billqtd = rest / 50;
            if billqtd > self.bank.fifty {
                rest -= self.bank.fifty * 50;
                resultbank.fifty = self.bank.fifty;
                self.bank.fifty = 0
            } else {
                rest -= billqtd * 50;
                resultbank.fifty = billqtd;
                self.bank.fifty -= billqtd;
            }
        }
        if rest >= 20 {
            let mut billqtd = rest / 20;
            if billqtd > self.bank.twenty {
                rest -= self.bank.twenty * 20;
                resultbank.twenty = self.bank.twenty;
                self.bank.twenty = 0
            } else {
                rest -= billqtd * 20;
                resultbank.twenty = billqtd;
                self.bank.twenty -= billqtd;
            }
        }
        if rest >= 10 {
            let mut billqtd = rest / 10;
            if billqtd > self.bank.ten {
                rest -= self.bank.ten * 10;
                resultbank.ten = self.bank.ten;
                self.bank.ten = 0
            } else {
                rest -= billqtd * 10;
                resultbank.ten = billqtd;
                self.bank.ten -= billqtd;
            }
        }
        if rest >= 5 {
            let mut billqtd = rest / 5;
            if billqtd > self.bank.five {
                rest -= self.bank.five * 5;
                resultbank.five = self.bank.five;
                self.bank.five = 0
            } else {
                rest -= billqtd * 5;
                resultbank.five = billqtd;
                self.bank.five -= billqtd;
            }
        }
        if rest >= 1 {
            if rest > self.bank.one {
                return Err(
                    "impossible to get this value the Atm dosent had the right bills to it!".into(),
                );
            } else {
                resultbank.one = rest;
                self.bank.one -= rest;
            }
        }

        Ok(resultbank)
    }
}

#[cfg(test)]
mod tests {
    use crate::machine;

    use super::*;
    #[test]
    fn withdraw() {
        let mut machine = AtmMachine::new();
        machine.deposit(150);
        machine.deposit(50);
        machine.deposit(50);
        println!("{:?}", machine.withdraw(220).unwrap());
    }
}
