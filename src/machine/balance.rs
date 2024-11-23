use crate::models::{bank::Bank, machine_model::AtmMachine};

impl AtmMachine {
    pub fn balance(self) -> Bank {
        self.bank.clone()
    }
}
