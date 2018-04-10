use std;

pub trait Proof : std::convert::AsRef<[u8]> {

}

#[cfg(test)]
use super::mocks;

#[cfg(test)]
mod tests {
    #[test]
    fn convert_to_slice() {
        let proof = mocks::MockProof { value: 42 };

        assert!(proof.as_ref() as i32 == 42)
    }
}