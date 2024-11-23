use crate::models::{bank::Bank, machine_model::AtmMachine};

impl AtmMachine {
    pub fn deposit(&mut self, value: i32) -> Bank {
        let mut bankresult = Bank::default();
        let mut rest = value;
        if rest >= 100 {
            let q = rest / 100;
            bankresult.onehundred = q as u32;
            rest = rest - q * 100;
        }
        if rest >= 50 {
            let q = rest / 50;
            bankresult.fifty = q as u32;
            rest = rest - q * 50;
        }
        if rest >= 20 {
            let q = rest / 20;
            bankresult.twenty = q as u32;
            rest = rest - q * 20;
        }
        if rest >= 10 {
            let q = rest / 10;
            bankresult.ten = q as u32;
            rest = rest - q * 10;
        }
        if rest >= 5 {
            let q = rest / 5;
            bankresult.five = q as u32;
            rest = rest - q * 5;
        }
        if rest >= 1 {
            let q = rest / 1;
            bankresult.one = q as u32;
        }
        self.bank = self.bank + bankresult;
        bankresult
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deposit() {
        let mut machine = AtmMachine::new();
        machine.deposit(789);
    }
}
