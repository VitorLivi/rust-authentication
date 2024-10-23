pub trait UseCase<I, O> {
    fn execute(&mut self, input: I) -> O;
}
