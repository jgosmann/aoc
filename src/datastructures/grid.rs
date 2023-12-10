use std::ops::{Index, IndexMut, Range};

pub struct GridView<T> {
    width: usize,
    pub_size: (usize, usize),
    data: T,
}

impl<T> GridView<Vec<T>> {
    pub fn from_vec(width: usize, separator_width: usize, data: Vec<T>) -> Self {
        if data.len() % width > 0 && data.len() % width < width - separator_width - 1 {
            panic!("width must be a divisor of total data length");
        }
        Self {
            width,
            pub_size: ((data.len() + width - 1) / width, width - separator_width),
            data,
        }
    }
}

impl<'a, T> GridView<&'a [T]> {
    pub fn new(width: usize, separator_width: usize, data: &'a [T]) -> Self {
        if data.len() % width > 0 && data.len() % width < width - separator_width - 1 {
            panic!("width must be a divisor of total data length");
        }
        Self {
            width,
            data,
            pub_size: ((data.len() + width - 1) / width, width - separator_width),
        }
    }

    pub fn from_separated(separator: T, data: &'a [T]) -> Self
    where
        T: Eq,
    {
        let width = data
            .iter()
            .position(|item| *item == separator)
            .unwrap_or(data.len());
        Self::new(width + 1, 1, data)
    }
}

impl<T> GridView<T> {
    pub fn size(&self) -> (usize, usize) {
        self.pub_size
    }

    pub fn width(&self) -> usize {
        self.pub_size.1
    }

    pub fn height(&self) -> usize {
        self.pub_size.0
    }

    pub fn nth_index(&self, n: usize) -> (usize, usize) {
        (n / self.width(), n % self.width())
    }
}

impl<T> GridView<T>
where
    GridView<T>: Index<(usize, usize)>,
    <GridView<T> as Index<(usize, usize)>>::Output: Copy + Sized,
{
    pub fn iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = <GridView<T> as Index<(usize, usize)>>::Output> + 'a {
        GridIterator::new(self)
    }
}

impl<T> Index<(usize, usize)> for GridView<&[T]> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.1 >= self.width() {
            panic!("index exceeds view dimensions");
        }
        self.data.index(self.width * index.0 + index.1)
    }
}

impl<T> Index<(usize, usize)> for GridView<&mut [T]> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.1 >= self.width() {
            panic!("index exceeds view dimensions");
        }
        self.data.index(self.width * index.0 + index.1)
    }
}

impl<T> IndexMut<(usize, usize)> for GridView<&mut [T]> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.1 >= self.width() {
            panic!("index exceeds view dimensions");
        }
        self.data.index_mut(self.width * index.0 + index.1)
    }
}

impl<T> Index<(usize, usize)> for GridView<Vec<T>> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.1 >= self.width() {
            panic!("index exceeds view dimensions");
        }
        self.data.index(self.width * index.0 + index.1)
    }
}

impl<T> IndexMut<(usize, usize)> for GridView<Vec<T>> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.1 >= self.width() {
            panic!("index exceeds view dimensions");
        }
        self.data.index_mut(self.width * index.0 + index.1)
    }
}

impl<T> Index<(usize, Range<usize>)> for GridView<&[T]> {
    type Output = [T];

    fn index(&self, index: (usize, Range<usize>)) -> &Self::Output {
        if index.1.end > self.width() {
            panic!("index exceeds view dimensions");
        }
        let row_start = self.width * index.0;
        self.data
            .index(row_start + index.1.start..row_start + index.1.end)
    }
}

struct GridIterator<'a, T> {
    grid: &'a GridView<T>,
    row: usize,
    col: usize,
}

impl<'a, T> GridIterator<'a, T> {
    pub fn new(grid: &'a GridView<T>) -> Self {
        Self {
            grid,
            row: 0,
            col: 0,
        }
    }
}

impl<'a, T> Iterator for GridIterator<'a, T>
where
    GridView<T>: Index<(usize, usize)>,
    <GridView<T> as Index<(usize, usize)>>::Output: Copy + Sized,
{
    type Item = <GridView<T> as Index<(usize, usize)>>::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.grid.height() {
            return None;
        }
        let current = self.grid[(self.row, self.col)];
        self.col += 1;
        if self.col >= self.grid.width() {
            self.col = 0;
            self.row += 1;
        }
        Some(current)
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use std::{ops::Range, vec};

    use super::GridView;

    static DATA: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    #[test]
    #[should_panic]
    fn test_invalid_grid_view_creation_panics() {
        GridView::new(5, 2, &DATA);
    }

    #[test]
    fn test_grid_view_creation_from_separated() {
        let grid = GridView::from_separated(b'\n', b"123\n456\n789");
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid[(2, 2)], b'9');
    }

    #[rstest]
    #[case(8)]
    #[case(9)]
    #[case(10)]
    fn test_grid_view_indexing(#[case] len: usize) {
        let grid = GridView::new(5, 2, &DATA[0..len]);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
        assert_eq!(grid[(0, 0)], 0);
        assert_eq!(grid[(0, 2)], 2);
        assert_eq!(grid[(1, 0)], 5);
        assert_eq!(grid[(1, 2)], 7);
    }

    #[rstest]
    #[case((0, 3))]
    #[case((2, 0))]
    #[case((3, 3))]
    #[should_panic]
    fn test_invalid_grid_view_indexing(#[case] index: (usize, usize)) {
        let grid = GridView::new(5, 2, &DATA[0..10]);
        grid[index];
    }

    #[test]
    fn test_grid_view_range_indexing() {
        let grid = GridView::new(5, 2, &DATA[0..10]);
        assert_eq!(grid[(0, 0..3)], [0, 1, 2]);
        assert_eq!(grid[(1, 1..2)], [6]);
    }

    #[rstest]
    #[case((0, 0..4))]
    #[case((2, 1..2))]
    #[should_panic]
    fn test_invalid_grid_view_range_indexing(#[case] index: (usize, Range<usize>)) {
        let grid = GridView::new(5, 2, &DATA[0..10]);
        let _ = &grid[index];
    }

    #[test]
    fn test_iterating_over_grid() {
        let grid = GridView::new(5, 2, &DATA[0..10]);
        let items: Vec<_> = grid.iter().collect();
        assert_eq!(items, vec![0, 1, 2, 5, 6, 7]);
    }

    #[test]
    fn test_nth_index() {
        let grid = GridView::new(5, 2, &DATA[0..10]);
        assert_eq!(grid.nth_index(5), (1, 2));
    }
}
