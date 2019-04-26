use std::ops::Add;
use std::fmt::{Formatter, Display, Result};

#[derive(Debug)]
struct Money<T> {
    amount: T,
    currency: String,
}

impl<T: Add<T, Output=T>> Add for Money<T> {
    type Output = Money<T>;
    fn add(self, rhs: Money<T>) -> Self::Output {
        assert!(self.currency == rhs.currency);
        Money { currency: rhs.currency, amount: self.amount + rhs.amount }
    }
}

impl<T: Display> Display for Money<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}

#[derive(Debug)]
struct CurrencylessMoney<T> {
    amount: T
}

impl<T> Into<CurrencylessMoney<T>> for Money<T> {
    fn into(self) -> CurrencylessMoney<T> {
        CurrencylessMoney { amount: self.amount }
    }
}

fn main() {
    let whole_euros: Money<u8> = Money { amount: 42, currency: "EUR".to_string() };
    let floating_euros: Money<f32> = Money { amount: 24.312, currency: "EUR".to_string() };

    println!("Whole euros: {:?}", whole_euros);
    println!("Floating euros: {:?}", floating_euros);

    let whole_euros_1: Money<u8> = Money { amount: 42, currency: "EUR".to_string() };
    let whole_euros_2: Money<u8> = Money { amount: 42, currency: "EUR".to_string() };
    let summed_euros = whole_euros_1 + whole_euros_2;

    println!("Summed euros: {:?}", summed_euros);

    let money = Money { amount: 42, currency: "EUR".to_string() };
    let currencyless_money: CurrencylessMoney<u32> = money.into();

    println!("Money without currency: {:?}", currencyless_money);

    let money = Money { amount: 42, currency: "EUR".to_string() };
    println!("Displaying money: {}", money);
}