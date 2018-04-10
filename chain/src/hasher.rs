use super::digest::FixedOutput;
use super::digest::generic_array::GenericArray;

use std;

pub struct Id<H: Hasher> {
    value: GenericArray<u8, <<H as Hasher>::Output as FixedOutput>::OutputSize>
}

impl<H: Hasher> std::ops::Deref for Id<H> {
    type Target = GenericArray<u8, <<H as Hasher>::Output as FixedOutput>::OutputSize>;

    #[inline]
    fn deref(&self) -> &GenericArray<u8, <<H as Hasher>::Output as FixedOutput>::OutputSize> {
        &self.value
    }
}

pub trait Hasher : std::default::Default {
    type Output : FixedOutput;

    fn hash<'a>(&mut self, input: &'a [u8]);
    fn finalize(self) -> Id<Self>;
}

#[cfg(test)]
use super::mocks;

#[cfg(test)]
mod tests {
    #[test]
    fn hash_int() {

    }

    #[test]
    fn hash_string() {

    }
}