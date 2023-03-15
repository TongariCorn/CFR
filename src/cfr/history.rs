pub trait History {
    type Info;

    fn get_info_set(&self) -> Self::Info;
}
