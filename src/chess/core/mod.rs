pub mod file;
pub mod rank;
pub mod square;

pub fn dance(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    // clockwise from North
    N, NE, E, SE, S, SW, W, NW
}

trait HasNeighbour<T> {

    fn neighbour(&self, dir: Direction) -> Option<T>;
}
