
use self::FileVal::*;
use self::RankVal::*;
// use self::Direction::*;

pub fn dance(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum FileVal {
    A, B, C, D, E, F, G, H
}


#[derive(PartialEq)]
#[derive(Debug)]
pub enum RankVal {
    One, Two, Three, Four, Five, Six, Seven, Eight
}


pub enum Direction {
    // clockwise from North
    N, NE, E, SE, S, SW, W, NW
}

pub struct File {
    pub value: FileVal
}

impl File {

    fn east_neighbour(&self) -> Option<File> {
        let new_value = match self.value {
            A => Some(B),
            B => Some(C),
            C => Some(D),
            D => Some(E),
            E => Some(F),
            F => Some(G),
            G => Some(H),
            H => None
        };
        new_value.map(|f| File { value: f })
    }

    fn west_neighbour(&self) -> Option<File> {
        let new_value = match self.value {
            A => None,
            B => Some(A),
            C => Some(B),
            D => Some(C),
            E => Some(D),
            F => Some(E),
            G => Some(F),
            H => Some(G)
        };
        new_value.map(|f| File { value: f })
    }

    pub fn neighbour(&self, dir: Direction) -> Option<File> {

        match dir {
            Direction::NE | Direction::E | Direction::SE => self.east_neighbour(),
            Direction::SW | Direction::W | Direction::NW => self.west_neighbour(),
            _ => None
        }
    }
}

pub struct Rank {
    pub value: RankVal
}

impl Rank {

    fn south_neighbour(&self) -> Option<Rank> {
        let new_value = match self.value {
            One => None,
            Two => Some(One),
            Three => Some(Two),
            Four => Some(Three),
            Five => Some(Four),
            Six => Some(Five),
            Seven => Some(Six),
            Eight => Some(Seven)
        };
        new_value.map(|r| Rank { value: r })
    }

    fn north_neighbour(&self) -> Option<Rank> {
        let new_value = match self.value {
            One => Some(Two),
            Two => Some(Three),
            Three => Some(Four),
            Four => Some(Five),
            Five => Some(Six),
            Six => Some(Seven),
            Seven => Some(Eight),
            Eight => None
        };
        new_value.map(|r| Rank { value: r } )
    }

    pub fn neighbour(&self, dir: Direction) -> Option<Rank> {

        match dir {
            Direction::N | Direction::NE | Direction::NW => self.north_neighbour(),
            Direction::S | Direction::SE | Direction::SW => self.south_neighbour(),
            _ => None
        }
    }
}

struct Square {
    file: File,
    rank: Rank
}

impl Square {

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn file_should_get_correct_neigbour() {
        let a = File { value: FileVal::A };
        if let Some(b) = a.neighbour(Direction::NE) {
            assert_eq!(FileVal::B, b.value);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn rank_should_get_correct_neigbour() {
        let one = Rank { value: RankVal::One };
        if let Some(two) = one.neighbour(Direction::NE) {
            assert_eq!(RankVal::Two, two.value);
        } else {
            assert!(false);
        }
    }
}
