pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        match self.row_count {
            0 => Vec::<Vec<u32>>::new(),
            1 => vec![vec![1]],
            _ => {
                let mut pascals = PascalsTriangle::new(self.row_count - 1).rows();
                let mut new_row: Vec<u32> = vec![1];
                let mut inner_row: Vec<u32> = pascals.last().unwrap().windows(2).map(|x| x[0] + x[1]).collect();
                new_row.append(&mut inner_row);
                new_row.push(1);
                pascals.push(new_row);
                pascals
            }
        }
    }
}
