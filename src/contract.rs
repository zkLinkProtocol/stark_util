pub trait Contract {
    type Handler;

    fn contract(&self) -> Self::Handler;
}
