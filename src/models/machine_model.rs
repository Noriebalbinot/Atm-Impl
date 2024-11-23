use super::bank::Bank;

#[derive(Debug, Default)]
pub struct AtmMachine {
    pub bank: Bank,
}

impl AtmMachine {
    pub fn new() -> AtmMachine {
        AtmMachine {
            ..Default::default()
        }
    }
}
