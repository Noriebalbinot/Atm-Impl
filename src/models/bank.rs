use std::ops::Add;

#[derive(Debug, Default, Clone, Copy)]
pub struct Bank {
    pub one: u32,
    pub five: u32,
    pub ten: u32,
    pub twenty: u32,
    pub fifty: u32,
    pub onehundred: u32,
}

impl Add for Bank {
    type Output = Bank;
    fn add(self, rhs: Self) -> Self::Output {
        Bank {
            onehundred: self.onehundred + rhs.onehundred,
            fifty: self.fifty + rhs.fifty,
            twenty: self.twenty + rhs.twenty,
            ten: self.ten + rhs.ten,
            five: self.five + rhs.five,
            one: self.one + rhs.one,
        }
    }
}

impl ToString for Bank {
    fn to_string(&self) -> String {
        let s = self.onehundred * 100
            + self.fifty * 50
            + self.twenty * 20
            + self.ten * 10
            + self.five * 5
            + self.one;
        format!("U$ {s}.00")
    }
}
