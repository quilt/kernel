#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Error<A> {
    MissingProof(A),
    MissingNode(u128),
}
