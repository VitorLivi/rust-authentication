pub trait UseCase<I, O> {
    fn execute(&self, input: I) -> O;
}
