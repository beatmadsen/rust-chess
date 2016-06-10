use super::Direction;
use super::HasNeighbour;
use self::RankVal::*;

#[derive(PartialEq, Debug)]
pub enum RankVal {
    One, Two, Three, Four, Five, Six, Seven, Eight
}

#[derive(PartialEq, Debug)]
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


}


impl HasNeighbour<Rank> for Rank {

    fn neighbour(&self, dir: super::Direction) -> Option<Rank> {

        match dir {
            Direction::N | Direction::NE | Direction::NW => self.north_neighbour(),
            Direction::S | Direction::SE | Direction::SW => self.south_neighbour(),
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
    fn rank_should_get_correct_neigbour() {
        let one = Rank { value: RankVal::One };
        if let Some(two) = one.neighbour(Direction::NE) {
            assert_eq!(RankVal::Two, two.value);
        } else {
            assert!(false);
        }
    }
}
