pub(crate) const ADDRESS_SIZE: usize = 1;
type T = [u8; ADDRESS_SIZE];

#[derive(Clone, Copy, Debug)]
pub struct Address(pub(crate) T);

impl Address {
    pub fn new(t: T) -> Self {
        Address(t)
    }

    pub fn zero() -> Self {
        Address([0u8; ADDRESS_SIZE])
    }
}
