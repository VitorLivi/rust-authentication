use actix_session::Session;

pub trait UseCase<I, O> {
    fn execute(&self, input: I) -> O;
}
