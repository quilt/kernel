#[derive(Debug)]
pub enum Error<A> {
    MissingProof(A),
}
