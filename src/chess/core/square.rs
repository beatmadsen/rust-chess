use super::file::File;
use super::rank::Rank;
use super::HasNeighbour;

#[derive(PartialEq, Debug)]
pub struct Square {
    pub file: File,
    pub rank: Rank
}

impl Square {

    pub fn neighbour(&self, dir: super::Direction) -> Option<Square> {

        let file_neighbour = || self.file.neighbour(dir);
        let rank_neighbour = |file| {
            self.rank.neighbour(dir).map(|rank| (file, rank))
        };

        file_neighbour()
            .and_then(rank_neighbour)
            .map(|(f, r)| Square { file: f, rank: r} )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::file::{File, FileVal};
    use super::super::rank::{Rank, RankVal};
    use super::super::Direction;
    use super::super::HasNeighbour;

    #[test]
    fn square_should_get_correct_neigbour() {

        let a = File { value: FileVal::A };
        let one = Rank { value: RankVal::One };
        let a_one = Square { file: a, rank: one };

        let b = File { value: FileVal::B };
        let two = Rank { value: RankVal::Two };
        let b_two = Square { file: b, rank: two };

        if let Some(x) = a_one.neighbour(Direction::NE) {
            assert_eq!(x, b_two);
        } else {
            assert!(false);
        }
    }
}
