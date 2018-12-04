pub trait Object {
    fn initialize(&mut self);
    fn dispose(&mut self);
    fn reset(&mut self);
    fn update(&mut self);
    fn place(&mut self);
}