#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn from_comma_separated(data_string: String) -> Point {
        let pair: Vec<usize> = data_string
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        Point {
            x: pair[0],
            y: pair[1],
        }
    }

    pub fn get_hash_key(&self) -> String {
        format!("{}x{}", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    pub fn from_arrowed_pair(data_string: &str) -> Line {
        let mut pair: Vec<Point> = data_string
            .split(" -> ")
            .map(|x| Point::from_comma_separated(x.to_string()))
            .collect();

        Line {
            a: pair.remove(0),
            b: pair.remove(0),
        }
    }

    pub fn create_line_series(&self, allow_diagonal: bool) -> Vec<Point> {
        let mut series: Vec<Point> = vec![];
        if self.a.x == self.b.x {
            let (mut start, mut end) = (self.a.y, self.b.y);
            if start > end {
                end = start;
                start = self.b.y;
            }
            for i in start..end + 1 {
                series.push(Point { x: self.a.x, y: i })
            }
        } else if self.a.y == self.b.y {
            let (mut start, mut end) = (self.a.x, self.b.x);
            if start > end {
                end = start;
                start = self.b.x;
            }
            for i in start..end + 1 {
                series.push(Point { x: i, y: self.a.y })
            }
        } else {
            if !allow_diagonal {
                return series;
            }
            let (mut reverse_x, mut reverse_y, mut start_x, mut end_x, mut start_y, mut end_y) =
                (false, false, self.a.x, self.b.x, self.a.y, self.b.y);

            if start_y > end_y {
                end_y = start_y;
                start_y = self.b.y;
                reverse_y = true;
            }
            if start_x > end_x {
                end_x = start_x;
                start_x = self.b.x;
                reverse_x = true;
            }

            let mut xs: Vec<usize> = (start_x..end_x + 1).collect();
            let mut ys: Vec<usize> = (start_y..end_y + 1).collect();

            if reverse_x {
                xs.reverse();
            }
            if reverse_y {
                ys.reverse();
            }
            for i in 0..ys.len() {
                series.push(Point { x: xs[i], y: ys[i] })
            }
        }
        series
    }
}
