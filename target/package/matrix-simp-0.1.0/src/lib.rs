use num_traits::{Float, Zero};
use std::{
        convert::From,
        fmt};

pub struct Matrix<T> {
    pub n: usize,
    pub m: usize,
    data: Vec<T> 
}

impl<T: Float + Zero + From<f32>> Matrix<T> {
    pub fn new(val: f32, n: usize, m:usize) -> Matrix<T> {
        Matrix { n , m , data: vec![<T as From<f32>>::from(val); n * m] }
    }
    pub fn zeros(n: usize, m:usize) -> Matrix<T> {
        Matrix { n , m , data: vec![<T as From<f32>>::from(0_f32); n * m] }
    }
    pub fn identity(n: usize) -> Matrix<T> {
        let mut mat: Matrix<T> = Matrix::zeros(n,n);
        for idx in 0..n {
            mat.data[(n+1) * idx] = <T as From<f32>>::from(1_f32); 
        }
        mat
    }
}

impl<T: Float + Zero + From<f32>> fmt::Display for Matrix<T> {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix<{} rows x {} cols>", self.n, self.m)
    }
}

impl<T: Float + Zero + From<f32> + fmt::Display> fmt::Debug for Matrix<T> {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix<{} rows x {} cols>:\n", self.n, self.m)?;
        for (idx, item) in self.data.iter().enumerate() {
            if idx%self.m == 0 {
                write!(f, "\n")?;
            }
            write!(f, "{item}\t")?;

        }

    Ok(())
    }
}

impl<T: Float + Zero + From<f32>> From<Vec<Vec<T>>> for Matrix<T> {

    fn from(value: Vec<Vec<T>>) -> Matrix<T> {
        
        let n = value.len();
        let m = value[0].len();

        let data: Vec<T> = value
                        .iter().fold(Vec::new(),|mut acc, n| {
                            acc.extend(n);
                            acc
                        });

        Matrix { n, m, data }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init_zeroes() {
        let mat: Matrix<f32> = Matrix::zeros(5, 4);
        assert_eq!(
            mat.data.into_iter().reduce(|acc, x| acc + x).unwrap(),
            0_f32)
    }

    #[test]
    fn init_val() {
        let mat: Matrix<f32> = Matrix::new(1_f32, 5, 4);
        assert_eq!(
            mat.data.into_iter().reduce(|acc, x| acc + x).unwrap(),
            20_f32)
    }

    #[test]
    fn test_from_2d_vec() {
        
        let mut data: Vec<Vec<f32>> = Vec::new();
        data.push(vec![1_f32,2_f32,3_f32]);
        data.push(vec![3_f32,2_f32,1_f32]);

        let mat: Matrix<f32> = Matrix::from(data);

        let comp: Vec<f32> = vec![1_f32,2_f32,3_f32,
                                    3_f32,2_f32,1_f32];


        assert_eq!(mat.data, comp);
        assert_eq!(mat.n, 2);
        assert_eq!(mat.m, 3);


    }

}
