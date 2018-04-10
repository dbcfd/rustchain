use super::sha2::{Sha256, Sha256Engine};

struct Sha2Hasher {
    h: Sha256Engine
}

impl Default for Sha2Hasher {
    fn default() -> Sha2Hasher {
        Sha2Hasher {
            h: Sha256::default()
        }
    }
}

impl Hasher<Sha256> for Sha2Hasher {
    type OutputSize = Sha256::OutputSize;

    fn hash(&mut self, input: &[u8]) {
        self.h.input(input);
    }
    fn finalize(self) -> GenericArray<u8, D::OutputSize> {
        self.h.finish();
        self.fixed_result()
    }
}