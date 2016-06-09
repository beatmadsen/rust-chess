pub mod file;
pub mod rank;

use self::file::File;
use self::rank::Rank;

pub fn dance(a: i32, b: i32) -> i32 {
    a + b
}


pub enum Direction {
    // clockwise from North
    N, NE, E, SE, S, SW, W, NW
}

struct Square {
    file: File,
    rank: Rank
}

impl Square {

}
