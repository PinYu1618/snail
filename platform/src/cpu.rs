pub(crate) trait Cpu {
    fn id() -> u8;

    fn frequency() -> u16;
}