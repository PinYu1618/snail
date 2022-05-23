pub struct CtrlHeader {
    pub type_: u32,
    pub flags: u32,
    pub fence_id: u64,
    pub ctx_id: u64,
    pub ring_idx: u8,
    _padding: [u8; 3],
}
