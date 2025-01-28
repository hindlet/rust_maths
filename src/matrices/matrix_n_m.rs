use std::ops::{Add, Deref, Mul, Sub};


/// I had to do this for my computational maths class
/// I refused to do it in python

#[derive(Debug)]
pub struct MatrixIndexError;


#[derive(Debug, Clone)]
pub struct MaxtrixNM {
    pub matrix: Vec<Vec<f32>>,
    pub width: usize,
    pub height: usize,
}

impl MaxtrixNM {
    pub fn new_empty(width: usize, height: usize) -> MaxtrixNM {
        MaxtrixNM {
            matrix: vec![vec![0.0; width]; height],
            width,
            height
        }
    }
    
    pub fn new_from_items(width: usize, height: usize, items: Vec<f32>) -> MaxtrixNM {
        let mut new = Self::new_empty(width, height);
        for i in 0..items.len() {
            new.matrix[i / width][i % width] = items[i];
        }
        new
    }


    pub fn get_row(&self, index: usize) -> Result<Vec<f32>, MatrixIndexError> {
        if index >= self.height {
            return Err(MatrixIndexError{});
        }
        Ok(self.matrix[index].clone())
    }

    pub fn get_column(&self, index: usize) -> Result<Vec<f32>, MatrixIndexError> {
        if index >= self.width {
            return Err(MatrixIndexError);
        }
        let mut column = Vec::new();
        for row in self.matrix.iter() {
            column.push(row[index]);
        }
        Ok(column)
    }

    pub fn get_element(&self, x: usize, y: usize) -> Result<f32, MatrixIndexError> {
        if x >= self.width || y >= self.height {
            return Err(MatrixIndexError{});
        }
        Ok(self.matrix[y][x])
    }


    pub fn determinant(&self) -> f32 {

    }

    pub fn transpose(&self) -> MaxtrixNM {

    }

}

#[derive(Debug)]
pub struct MatrixArithmeticError;


impl Sub for MaxtrixNM {
    type Output = Result<MaxtrixNM, MatrixArithmeticError>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.width != rhs.width || self.height != rhs.height {
            return Err(MatrixArithmeticError);
        }
        let mut result = MaxtrixNM::new_empty(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                result.matrix[y][x] = self.matrix[y][x] - rhs.matrix[y][x];
            }
        }
        Ok(result)
    }
}

impl Add for MaxtrixNM {
    type Output = Result<MaxtrixNM, MatrixArithmeticError>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.width != rhs.width || self.height != rhs.height {
            return Err(MatrixArithmeticError);
        }
        let mut result = MaxtrixNM::new_empty(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                result.matrix[y][x] = self.matrix[y][x] + rhs.matrix[y][x];
            }
        }
        Ok(result)
    }
}

impl Mul for MaxtrixNM {
    type Output = Result<MaxtrixNM, MatrixArithmeticError>;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.width != rhs.height {
            return Err(MatrixArithmeticError{});
        }
        let mut items = Vec::new();
        for y in 0..self.height {
            for x in 0..rhs.width {
                let mut total = 0.0;
                for i in 0..self.width {
                    total += self.get_element(i, y).unwrap() * rhs.get_element(x, i).unwrap();
                }
                items.push(total);
            }
        }

        Ok(MaxtrixNM::new_from_items(rhs.width, self.height, items))
    }
}

impl Mul<f32> for MaxtrixNM {
    type Output = MaxtrixNM;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut items = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                items.push(self.get_element(x, y).unwrap() * rhs);
            }
        }

        MaxtrixNM::new_from_items(self.width, self.height, items)
    }
}