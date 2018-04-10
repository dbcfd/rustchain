use hasher::{Hasher, Id};
use transactions::Transactions;
use proof::Proof;

struct Block<H: Hasher, V: Transactions, P: Proof> {
    index: usize,
    previous: Option<Id<H>>,
    proof: P,
    transactions: V,
    id: Id<H>
}

impl<H: Hasher, V: Transactions, P: Proof> Block<H, V, P> {
    fn genesis<H2: Hasher, V2: Transactions, P2: Proof>(
        proof: P2,
        transactions: V2
    ) -> Block<H2, V2, P2> {
        let mut hasher = H2::default();

        hasher.hash(proof.as_ref());
        transactions.vec().iter().for_each(|e| {
            hasher.hash(e.as_ref());
        });

        let id = hasher.finalize();

        Block {
            index: 0,
            previous: None,
            proof: proof,
            transactions: transactions,
            id: id
        }
    }

    fn next<H2: Hasher, V2: Transactions, P2: Proof>(
        previous: Block<H2, V2, P2>,
        proof: P2,
        transactions: V2
    ) -> Block<H2, V2, P2> {
        let mut hasher = H2::default();

        hasher.hash(previous.id.as_ref());
        hasher.hash(proof.as_ref());
        transactions.vec().iter().for_each(|e| {
          hasher.hash(e.as_ref());
        });

        let id = hasher.finalize();

        Block {
            index: previous.index + 1,
            previous: Some(previous.id),
            proof: proof,
            transactions: transactions,
            id: id
        }
    }
}

#[cfg(test)]
use super::mocks;

#[cfg(test)]
mod tests {
    #[test]
    fn create_blocks() {
        Hasher::genesis(
            mocks::DefaultProof { value: 42 },
            mocks::MockTransactions {
                vec: std::vec![
                    mocks::MockTransaction { value: 100 },
                    mocks::MockTransaction { value: 200 }
                ]
            }
        )
    }
}