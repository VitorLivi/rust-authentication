use actix_session::Session;

pub trait UseCase<I, O> {
    fn new() -> Self;
    fn execute(&self, input: I) -> O;
}
