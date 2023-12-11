#![allow(dead_code)]

type Point = (isize, isize);

pub struct Grid<T> {
    pub width: isize,
    pub height: isize,
    data: Box<[T]>,
}

impl<T> Grid<T> {
    fn is_out_of_bound(self: &Self, (x, y): Point) -> bool {
        x < 0 || x >= self.width || y < 0 || y >= self.height
    }

    pub fn get(&self, (x, y): Point) -> Option<&T> {
        if self.is_out_of_bound((x, y)) {
            None
        } else {
            Some(&self.data[(y * self.width + x) as usize])
        }
    }

    pub fn get_mut(&mut self, (x, y): Point) -> Option<&mut T> {
        if self.is_out_of_bound((x, y)) {
            None
        } else {
            Some(&mut self.data[(y * self.width + x) as usize])
        }
    }
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(width: isize, height: isize) -> Grid<T> {
        Grid {
            width,
            height,
            data: vec![Default::default(); (width * height) as usize].into_boxed_slice(),
        }
    }

    pub fn from_nested(input: &[Vec<T>]) -> Grid<T> {
        let width = input.first().unwrap().len();
        let height = input.len();

        let mut data = vec![Default::default(); width * height].into_boxed_slice();
        for (y, row) in input.iter().enumerate() {
            let begin = y * width;
            data[begin..begin + width].clone_from_slice(row);
        }

        Grid {
            width: width as isize,
            height: height as isize,
            data,
        }
    }

    pub fn rows(self: &Self) -> Vec<&[T]> {
        let mut res = vec![];
        for y in 0..self.height {
            let begin = (y * self.width) as usize;
            let end = begin + self.width as usize;
            res.push(&self.data[begin..end]);
        }
        res
    }
}

impl<T> std::ops::Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        self.get(point).unwrap()
    }
}

impl<T> std::ops::IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        self.get_mut(point).unwrap()
    }
}
