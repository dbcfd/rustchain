use std;

pub trait Transactions {
    type Transaction : std::convert::AsRef<[u8]>;

    fn vec(&self) -> std::vec::Vec<&Self::Transaction>;
}

#[cfg(test)]
use super::mocks;

#[cfg(test)]
mod tests {
    #[test]
    fn returns_vec() {
        let transactions = mocks::MockTransactions {
            vec: std::vec![mocks::MockTransaction { value: 42 }, mocks::MockTransaction { value: 43 }]
        };

        assert_eq!(transactions.vec().length, 2)
    }
}