const TOP: u8 = 0b1000;
const RIGHT: u8 = 0b0100;
const BOTTOM: u8 = 0b0010;
const LEFT: u8 = 0b0001;

pub struct SurroundIterator2d {
    center: (usize, usize),
    sides: u8,
    index: usize,
}

impl SurroundIterator2d {
    pub fn new(center: (usize, usize), size: (usize, usize)) -> Self {
        let top = if center.0 > 0 { TOP } else { 0 };
        let bottom = if center.0 < size.0 - 1 { BOTTOM } else { 0 };
        let left = if center.1 > 0 { LEFT } else { 0 };
        let right = if center.1 < size.1 - 1 { RIGHT } else { 0 };
        Self {
            center,
            sides: top | right | bottom | left,
            index: 0,
        }
    }

    fn current(&self) -> Option<(usize, usize)> {
        match self.index {
            1 if self.sides & TOP != 0 && self.sides & LEFT != 0 => {
                Some((self.center.0 - 1, self.center.1 - 1))
            }
            2 if self.sides & TOP != 0 => Some((self.center.0 - 1, self.center.1)),
            3 if self.sides & TOP != 0 && self.sides & RIGHT != 0 => {
                Some((self.center.0 - 1, self.center.1 + 1))
            }
            4 if self.sides & LEFT != 0 => Some((self.center.0, self.center.1 - 1)),
            5 if self.sides & RIGHT != 0 => Some((self.center.0, self.center.1 + 1)),
            6 if self.sides & BOTTOM != 0 && self.sides & LEFT != 0 => {
                Some((self.center.0 + 1, self.center.1 - 1))
            }
            7 if self.sides & BOTTOM != 0 => Some((self.center.0 + 1, self.center.1)),
            8 if self.sides & BOTTOM != 0 && self.sides & RIGHT != 0 => {
                Some((self.center.0 + 1, self.center.1 + 1))
            }
            _ => None,
        }
    }
}

impl Iterator for SurroundIterator2d {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = None;
        while self.index < 9 && current.is_none() {
            self.index += 1;
            current = self.current()
        }
        current
    }
}

#[cfg(test)]
mod test {
    use super::SurroundIterator2d;

    #[test]
    fn test_surround_iterator_2d_upper_left() {
        let indices: Vec<_> = SurroundIterator2d::new((0, 0), (3, 3)).collect();
        assert_eq!(indices, vec![(0, 1), (1, 0), (1, 1)]);
    }

    #[test]
    fn test_surround_iterator_2d_lower_right() {
        let indices: Vec<_> = SurroundIterator2d::new((2, 2), (3, 3)).collect();
        assert_eq!(indices, vec![(1, 1), (1, 2), (2, 1)]);
    }

    #[test]
    fn test_surround_iterator_2d_middle() {
        let indices: Vec<_> = SurroundIterator2d::new((1, 1), (3, 3)).collect();
        assert_eq!(
            indices,
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2)
            ]
        );
    }
}
