pub trait Widget {
    fn set_size();

    fn get_size() -> (u32, u32);

    fn set_pos();

    fn get_pos() -> (u32, u32);

    fn render();

    fn update();
}