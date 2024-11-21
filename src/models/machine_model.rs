use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct AtmMachine<'a> {
    bank: HashMap<&'a str, i32>,
}

impl<'a> AtmMachine<'a> {
    pub fn new() -> AtmMachine<'a> {
        AtmMachine {
            ..Default::default()
        }
    }
}
