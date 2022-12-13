use std::fmt::{self, Debug};

pub struct Faux2DArray<T> {
    pub width: usize,
    pub items: Vec<T>,
}

impl<T: Debug> fmt::Display for Faux2DArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        self.items.chunks(self.width).for_each(|l| {
            writeln!(f, "{:?}", l).expect("Cannot print line");
        });
        Ok(())
    }
}

impl<T: Debug> Debug for Faux2DArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        self.items.chunks(self.width).for_each(|l| {
            writeln!(f, "{:?}", l).expect("Cannot print line");
        });
        Ok(())
    }
}

impl<T> FromIterator<T> for Faux2DArray<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut s = Self {
            items: Vec::from_iter(iter),
            width: 0,
        };
        s.width = s.items.len();
        s
    }
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

    pub fn cartesian_index(&self, pos: usize) -> (usize, usize) {
        let y = pos / self.width;
        let x = pos % self.width;
        (x, y)
    }

    pub fn filled<F>(width: usize, height: usize, filler: F) -> Faux2DArray<T>
    where
        F: Fn(usize, usize) -> T,
    {
        let mut items = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                items.push(filler(x, y));
            }
        }
        Self { items, width }
    }

    pub fn height(&self) -> usize {
        self.items.len() / self.width
    }

    pub fn add_row(&mut self, data: Vec<T>) -> std::result::Result<(), &'static str> {
        if data.len() != self.width {
            return Err("Invalid row length");
        }
        self.items.extend(data);
        Ok(())
    }

    pub fn remove_row(&mut self, row: usize) -> std::result::Result<(), &'static str> {
        if row >= self.width {
            return Err("Invalid row length");
        }
        let start = row * self.width;
        let end = start + self.width;
        self.items.drain(start..end);
        Ok(())
    }

    pub fn add_col(&mut self, data: Vec<T>) -> std::result::Result<(), &'static str> {
        let data_length = data.len();
        if data_length != self.height() {
            return Err("Invalid column length");
        }
        for (i, d) in data.into_iter().enumerate() {
            let position = (self.width * (i + 1)) + i;
            self.items.splice(position..position, [d]);
        }
        self.width += 1;
        Ok(())
    }

    pub fn remove_col(&mut self, col: usize) -> std::result::Result<(), &'static str> {
        if col >= self.height() {
            return Err("Invalid column length");
        }
        for i in (0..self.height()).rev() {
            let position = col + (self.width * i);
            println!("{:?}", position);
            self.items.remove(position);
        }
        self.width -= 1;
        Ok(())
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height() {
            return None;
        }
        Some(&self.items[self.absolute_index(x, y)])
    }

    pub fn next_x(&self, x: usize, y: usize) -> Option<&T> {
        if x + 1 >= self.width {
            return None;
        }
        Some(&self.items[self.absolute_index(x + 1, y)])
    }

    pub fn prev_x(&self, x: usize, y: usize) -> Option<&T> {
        if x == 0 || x > self.width {
            return None;
        }
        Some(&self.items[self.absolute_index(x - 1, y)])
    }

    pub fn next_y(&self, x: usize, y: usize) -> Option<&T> {
        if y + 1 >= self.height() {
            return None;
        }
        Some(&self.items[self.absolute_index(x, y + 1)])
    }

    pub fn prev_y(&self, x: usize, y: usize) -> Option<&T> {
        if y == 0 || y > self.height() {
            return None;
        }
        Some(&self.items[self.absolute_index(x, y - 1)])
    }

    pub fn insert(&mut self, x: usize, y: usize, val: T) {
        let idx = self.absolute_index(x, y);
        self.items[idx] = val;
    }

    pub fn row(&self, row: usize) -> Option<impl Iterator<Item = &T> + '_> {
        let start = self.width * row;
        let end = start + self.width;
        Some(self.items[start..end].iter())
    }

    pub fn col(&self, col: usize) -> Option<impl Iterator<Item = &T> + '_> {
        if col >= self.width {
            return None;
        }
        Some(self.items.iter().skip(col).step_by(self.width))
    }

    pub fn rows(&self) -> impl Iterator<Item = Vec<&T>> + '_ {
        self.items
            .chunks(self.width)
            .map(|c| c.iter().collect::<Vec<&T>>())
    }

    pub fn cols(&self) -> impl Iterator<Item = Vec<&T>> + '_ {
        // self.items.
        (0..self.width).map(|idx| self.col(idx).unwrap().collect::<Vec<&T>>())
    }

    pub fn to_row_end(&self, x: usize, y: usize) -> Option<impl Iterator<Item = &T> + '_> {
        if x >= self.width {
            return None;
        }
        let row_start = self.width * y;
        let start = row_start + x + 1;
        let end = row_start + self.width;
        Some(self.items[start..end].iter())
    }

    pub fn to_row_start(&self, x: usize, y: usize) -> Option<impl Iterator<Item = &T> + '_> {
        if x >= self.width {
            return None;
        }
        let start = self.width * y;
        let end = start + x;
        Some(self.items[start..end].iter().rev())
    }

    pub fn to_col_end(&self, x: usize, y: usize) -> Option<impl Iterator<Item = &T> + '_> {
        if x >= self.width {
            return None;
        }
        Some(
            self.items[x + self.width + (y * self.width)..]
                .iter()
                .step_by(self.width),
        )
    }

    pub fn to_col_start(&self, x: usize, y: usize) -> Option<impl Iterator<Item = &T> + '_> {
        if x >= self.width {
            return None;
        }

        Some(
            self.items
                [..self.items.len() - ((self.width - 1 - x) + (self.width * (self.height() - y)))]
                .iter()
                .rev()
                .step_by(self.width),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_usize_array() -> Faux2DArray<usize> {
        let new_2d_array: Faux2DArray<usize> = Faux2DArray::filled(5, 5, |x, _| x);
        new_2d_array
    }
    fn create_usize_array_2() -> Faux2DArray<usize> {
        let new_2d_array: Faux2DArray<usize> = Faux2DArray::filled(5, 5, |x, y| (y * 5) + x);
        new_2d_array
    }

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
        assert_eq!(new_2d_array.at(0, 0).unwrap(), &"0x0".to_string());
        assert_eq!(new_2d_array.at(4, 4).unwrap(), &"4x4".to_string());
    }

    #[test]
    fn test_at() {
        let a = create_usize_array();
        assert_eq!(a.at(0, 0).unwrap(), &0);
        assert_eq!(a.at(3, 0).unwrap(), &3);
        assert_eq!(a.at(3, 4).unwrap(), &3);
    }

    #[test]
    fn test_row() {
        let a = create_usize_array();
        let target = vec![&0, &1, &2, &3, &4];
        assert_eq!(a.row(0).unwrap().collect::<Vec<&usize>>(), target);
    }

    #[test]
    fn test_rows() {
        let a = create_usize_array();
        let target: Vec<&usize> = vec![&0, &1, &2, &3, &4];
        let mut rows = a.rows();
        assert_eq!(rows.next().unwrap(), target);
        assert_eq!(rows.next().unwrap(), target);
        assert_eq!(rows.next().unwrap(), target);
        assert_eq!(rows.next().unwrap(), target);
    }

    #[test]
    fn test_col() {
        let a = create_usize_array();
        let target = vec![&0, &0, &0, &0, &0];
        let target2 = vec![&3, &3, &3, &3, &3];
        assert_eq!(a.col(0).unwrap().collect::<Vec<&usize>>(), target);
        assert_eq!(a.col(3).unwrap().collect::<Vec<&usize>>(), target2);
    }

    #[test]
    fn test_cols() {
        let a = create_usize_array();
        let target = vec![&0, &0, &0, &0, &0];
        let target1 = vec![&1, &1, &1, &1, &1];
        let target2 = vec![&2, &2, &2, &2, &2];
        let target3 = vec![&3, &3, &3, &3, &3];
        let target4 = vec![&4, &4, &4, &4, &4];
        let mut cols = a.cols();
        assert_eq!(cols.next().unwrap(), target);
        assert_eq!(cols.next().unwrap(), target1);
        assert_eq!(cols.next().unwrap(), target2);
        assert_eq!(cols.next().unwrap(), target3);
        assert_eq!(cols.next().unwrap(), target4);
    }

    #[test]
    fn test_add_row() {
        let mut a = create_usize_array();
        let result = a.add_row(vec![8, 8, 8, 8, 8]);
        assert!(result.is_ok(), "Row append failed!");
        let target = vec![&8, &8, &8, &8, &8];
        let row = a.row(5).unwrap().collect::<Vec<&usize>>();
        assert_eq!(row, target);
    }

    #[test]
    fn test_add_column() {
        let mut a = create_usize_array();
        let result = a.add_col(vec![8, 8, 8, 8, 8]);
        assert!(result.is_ok(), "Col append failed!");
        let target = vec![&8, &8, &8, &8, &8];
        let row = a.col(5).unwrap().collect::<Vec<&usize>>();
        assert_eq!(row, target);
    }

    #[test]
    fn test_remove_row() {
        let mut a = create_usize_array();
        let result = a.remove_row(0);
        assert!(result.is_ok(), "Row remove failed!");
        assert_eq!(a.items.len(), 20);
    }

    #[test]
    fn test_remove_column() {
        let mut a = create_usize_array();
        let result = a.remove_col(0);
        assert!(result.is_ok(), "Col remove failed!");
        assert_eq!(a.items.len(), 20);
    }

    #[test]
    fn test_to_row_start() {
        let a = create_usize_array_2();
        let result: Vec<&usize> = a.to_row_start(2, 0).unwrap().collect();
        let target = [&1, &0];
        assert_eq!(result.as_slice(), target);
    }

    #[test]
    fn test_to_row_end() {
        let a = create_usize_array_2();
        let result: Vec<&usize> = a.to_row_end(2, 0).unwrap().collect();
        let target = [&3, &4];
        assert_eq!(result.as_slice(), target);
    }

    #[test]
    fn test_to_col_start() {
        let a = create_usize_array_2();
        // println!("{}", a);
        // let len = a.items.len();
        // println!(
        //     "{:?}",
        //     a.items[..len - ((a.width - 1 - 1) + (a.width * (a.height() - 1)))]
        //         .iter()
        //         .collect::<Vec<&usize>>()
        // );
        let result: Vec<&usize> = a.to_col_start(1, 1).unwrap().collect();
        let target = [&1];
        assert_eq!(result.as_slice(), target);

        let result: Vec<&usize> = a.to_col_start(0, 2).unwrap().collect();
        let target = [&5, &0];
        assert_eq!(result.as_slice(), target);
    }

    #[test]
    fn test_to_col_end() {
        let a = create_usize_array_2();
        let result: Vec<&usize> = a.to_col_end(0, 2).unwrap().collect();
        let target = [&15, &20];
        assert_eq!(result.as_slice(), target);
    }
}
