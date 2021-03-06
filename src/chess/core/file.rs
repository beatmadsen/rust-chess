use super::Direction;
use super::HasNeighbour;
use self::FileVal::*;

#[derive(PartialEq, Debug)]
pub enum FileVal {
    A, B, C, D, E, F, G, H
}

#[derive(PartialEq, Debug)]
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
}

impl HasNeighbour<File> for File {

    fn neighbour(&self, dir: super::Direction) -> Option<File> {

        match dir {
            Direction::NE | Direction::E | Direction::SE => self.east_neighbour(),
            Direction::SW | Direction::W | Direction::NW => self.west_neighbour(),
            _ => None
        }
    }
}



#[cfg(test)]
mod tests {

    use super::*;
    use super::super::Direction;
    use super::super::HasNeighbour;

    #[test]
    fn file_should_get_correct_neigbour() {
        let a = File { value: FileVal::A };
        if let Some(b) = a.neighbour(Direction::NE) {
            assert_eq!(FileVal::B, b.value);
        } else {
            assert!(false);
        }
    }
}
