// 在mod内生命mod 以及添加函数
mod hosting {
    pub fn add_to_waitlist() -> i8 {
        10
    }

    // unused
    fn _seat_at_table() {}
}

use crate::chapter::package_crates::hosting::add_to_waitlist;

pub fn main() -> i8 {
    add_to_waitlist()
}