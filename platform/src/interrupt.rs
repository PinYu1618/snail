pub trait Interrupt {
    fn enable();

    fn disable();
}