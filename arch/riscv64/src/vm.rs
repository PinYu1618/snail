pub fn activate() {
    todo!()
}

pub fn current_token() {
    todo!()
}

bitflags::bitflags! {
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

bitflags::bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;    // Valid, 1 = valid
        const R = 1 << 1;    // Read
        const W = 1 << 2;    // Write
        const X = 1 << 3;    // eXecute
        const U = 1 << 4;    // User
        const G = 1 << 5;    // (dont know)
        const A = 1 << 6;    // Accessed
        const D = 1 << 7;    // Dirty
    }
}