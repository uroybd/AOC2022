use std::option::Iter;

pub struct Faux2DArray<T> {
    items: Vec<T>,
    width: usize,
}

impl<T> Faux2DArray<T> {
    pub fn new(width: usize) -> Faux2DArray<T> {
        Self {
            items: vec![],
            width,
        }
    }

    pub fn absolute_index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    pub fn filled<F>(width: usize, height: usize, filler: F) -> Faux2DArray<T>
    where
        F: Fn(usize, usize) -> T,
    {
        let mut items = vec![];
        for y in 0..height {
            for x in 0..width {
                items.push(filler(x, y));
            }
        }
        Self { items, width }
    }

    pub fn at(&self, x: usize, y: usize) -> &T {
        &self.items[self.absolute_index(x, y)]
    }

    pub fn insert(&mut self, x: usize, y: usize, val: T) {
        let idx = self.absolute_index(x, y);
        self.items[idx] = val;
    }

    pub fn row(&self, row: usize) -> impl Iterator<Item = &T> + '_ {
        let start = self.width * row;
        let end = start + self.width + 1;
        self.items[start..end].iter()
    }

    pub fn col(&self, col: usize) -> impl Iterator<Item = &T> + '_ {
        self.items.iter().skip(col - 1).step_by(self.width)
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> + '_ {
        self.items.chunks(self.width)
    }

    // pub fn cols(&self) -> impl Iterator<Item = &[T]> + '_ {
    //     // self.items.
    //     (0..self.width).map(|idx| {
    //         let items = self.col(idx).cloned().collect::<Vec<&T>>();
    //         items.clone().as_slice()
    //     })
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let new_2d_array: Faux2DArray<usize> = Faux2DArray::new(5);
        assert_eq!(new_2d_array.items.len(), 0);
        assert_eq!(new_2d_array.width, 5)
    }

    #[test]
    fn test_filled_with() {
        let new_2d_array: Faux2DArray<String> =
            Faux2DArray::filled(5, 5, |x, y| format!("{}x{}", x, y));
        assert_eq!(new_2d_array.items.len(), 25);
        assert_eq!(new_2d_array.at(0, 0), &"0x0".to_string());
        assert_eq!(new_2d_array.at(4, 4), &"4x4".to_string());
    }
}
