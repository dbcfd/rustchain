extern crate digest;
#[cfg(test)]
extern crate sha2;

pub mod block;
pub mod hasher;
pub mod proof;
pub mod transactions;

#[cfg(test)]
use sha2;

#[cfg(test)]
pub mod mocks {
    struct MockProof {
        value: i32
    }

    impl Proof for MockProof {

    }

    impl std::AsRef<u8> for MockProof {
        #[inline]
        fn as_ref(&self) -> &[u8] {
            self.value.as_ref()
        }
    }

    struct MockTransaction {
        value: i32
    }

    impl std::AsRef<u8> for MockTransaction {
        #[inline]
        fn as_ref(&self) -> &[u8] {
            self.value.as_ref()
        }
    }

    struct MockTransactions {
        vec: std::vec::Vec<MockTransaction>
    }

    impl Transactions for MockTransactions {
        type Transaction = MockTransaction;

        fn vec(&self) -> std::vec::Vec<&Self::Transaction> {
            self.vec
        }
    }

    struct MockHasher {
        h: sha2::Sha256Engine
    }

    impl Hasher for MockHasher {
        type Digest = Sha256;

        fn hash<'a>(&mut self, input: &'a [u8]) {
            self.h.input(input);
        }
        fn finalize(self) -> GenericArray<u8, Self::Digest::OutputSize> {
            self.h.finish();
            self.h.fixed_result()
        }
    }

    impl Default for MockHasher {
        fn default() -> MockHasher {
            MockHasher {
                h: sha2::Sha256::default()
            }
        }
    }
}