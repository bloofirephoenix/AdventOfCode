use std::ops::{Index, IndexMut};

pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T> {
        Grid {
            width,
            height,
            data: vec![T::default(); width * height]
        }
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        x + y*self.width
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[self.get_index(index.0, index.1)]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let i = self.get_index(index.0, index.1);
        &mut self.data[i]
    }
}