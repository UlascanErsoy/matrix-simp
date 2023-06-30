use num_traits::{Float, Zero};
use std::{
        convert::From,
        fmt,
        ops,
        cmp::max};

#[derive(Clone)]
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

    pub fn get_row(&self, row: usize) -> &[T] {
        &self.data[row * self.m..(row + 1) * self.m]
    }

    pub fn get_col(&self, col: usize) -> Vec<T> {
        self.data.iter()
                 .enumerate()
                 .filter(|&(idx, _)| idx%self.m == col)
                 .map(|(_ , num)| *num)
                 .collect::<Vec<T>>()
    }

    pub fn transpose(&mut self) -> Matrix<T> {

        let data: Vec<Vec<T>> = (0..self.m).map(|x| self.get_col(x))
                                           .collect::<Vec<Vec<T>>>();
        
        Matrix::from(data)
    }

    pub fn exp(&mut self) -> Matrix<T> {

        let data: Vec<T> = self.data.iter()
                               .map(|x| x.exp())
                               .collect::<Vec<T>>();
        
        Matrix { n:self.n , m:self.m , data }
        
    }

    pub fn one_over(&self) -> Matrix<T> {

        let data: Vec<T> = self.data.iter()
                               .map(|x| T::one() / *x)
                               .collect::<Vec<T>>();
        
        Matrix { n:self.n , m:self.m , data }

    }
}

impl<T: Float + Zero + From<f32>> fmt::Display for Matrix<T> {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix<{} rows x {} cols>", self.n, self.m)
    }
}



impl<T: Float + Zero + From<f32> + fmt::Display> fmt::Debug for Matrix<T> {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix<{} rows x {} cols>:", self.n, self.m)?;
        for (idx, item) in self.data.iter().enumerate() {
            if idx%self.m == 0 {
                write!(f, "\n")?;
            }
            write!(f, "{item}\t")?;

        }

    Ok(())
    }
}

impl<T: Float + Zero + From<f32> + fmt::Display + ops::Add> ops::Add<Matrix<T>> for Matrix<T> {
   
    type Output = Matrix<T>;

    fn add(self, _rhs: Matrix<T>) -> Matrix<T> {
        
       let mut mat: Matrix<_> = self.clone();

       for (idx, item) in mat.data.iter_mut().enumerate() {
            *item = *item + _rhs.data[idx]; 
       }

       mat
    }
}

impl<T: Float + Zero + From<f32> + fmt::Display + fmt::Debug + ops::Mul> ops::Mul<T> for Matrix<T> {

    type Output = Matrix<T>;

    fn mul(self, _rhs: T) -> Matrix<T> {

        let data: Vec<T> = self.data.iter()
                                    .map(|x| *x * _rhs)
                                    .collect::<Vec<T>>();

        
        Matrix { n: self.n, m: self.m, data }
    }

}

impl<T: Float + Zero + From<f32> + fmt::Display + fmt::Debug + ops::Div> ops::Div<T> for Matrix<T> {

    type Output = Matrix<T>;

    fn div(self, _rhs: T) -> Matrix<T> {

        let data: Vec<T> = self.data.iter()
                                    .map(|x| *x / _rhs)
                                    .collect::<Vec<T>>();

        
        Matrix { n: self.n, m: self.m, data }
    }

}

impl<T: Float + Zero + From<f32> + fmt::Display + fmt::Debug + ops::Mul> ops::Mul<Matrix<T>> for Matrix<T> {
   
    type Output = Matrix<T>;

    fn mul(self, _rhs: Matrix<T>) -> Matrix<T> {
        
       let mut mat: Matrix<T> = Matrix::zeros(
                                              max(_rhs.n, self.n),
                                              max(self.m, _rhs.m));

       for rdx in 0..self.n {
           for cdx in 0.._rhs.m {
               let row = self.get_row(rdx);
               let col = _rhs.get_col(cdx);

               let sum = row.into_iter()
                            .enumerate()
                            .map(|(idx, num)| {
                                *num * col[idx]
                            })
                            .reduce(|acc, x| acc + x)
                            .unwrap();
                
                
                    mat.data[rdx * mat.n + cdx] = sum;      
           }
       }
       mat
    }
}


impl<T> From<Vec<Vec<T>>> for Matrix<T> where 
T : Float + Zero + From<f32> {

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

impl<T: Float + Zero + From<f32>> From<&[&[T]]> for Matrix<T> {

    fn from(value: &[&[T]]) -> Matrix<T> {
        
        let n = value.len();
        let m = value[0].len();

        let data: Vec<T> = value
                        .iter().fold(Vec::new(),|mut acc, n| {
                            acc.extend(*n);
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

    #[test]
    fn test_from_2d_slice() {
        
        let data: &[&[f32]] = &[&[1_f32,0_f32],
                                  &[0_f32,1_f32]];

        let mat: Matrix<f32> = Matrix::from(data);

        let comp: Vec<f32> = vec![1_f32,0_f32,
                                    0_f32,1_f32];

        assert_eq!(mat.data, comp);
        assert_eq!(mat.n, 2);
        assert_eq!(mat.m, 2);


    }

    #[test]
    fn test_from_reg_add() {
        
        let data: &[&[f32]] = &[&[1_f32,0_f32],
                                  &[0_f32,1_f32]];

        let data2: &[&[f32]] = &[&[0_f32,1_f32],
                                  &[1_f32,0_f32]];

        let mat: Matrix<f32> = Matrix::from(data);
        let mat2: Matrix<f32> = Matrix::from(data2);

        let comp: Vec<f32> = vec![1_f32,1_f32,
                                    1_f32,1_f32];
        
        let res: Matrix<f32> = mat + mat2;

        assert_eq!(res.data, comp);

    }

    #[test]
    fn test_get_row() {
        let data: &[&[f32]] = &[&[1_f32,0_f32, 0_f32],
                                &[0_f32,1_f32, 3_f32],
                                &[2_f32,9_f32, 3_f32],
                                &[0_f32,1_f32, 3_f32]];

        let mat: Matrix<f32> = Matrix::from(data);

        let comp: &[f32] = &[2_f32, 9_f32, 3_f32];
        
        assert_eq!(
                    mat.get_row(2),
                    comp 
                );

    }

    #[test]
    fn test_get_col() {
        let data: &[&[f32]] = &[&[1_f32,0_f32, 0_f32],
                                &[0_f32,1_f32, 3_f32],
                                &[2_f32,9_f32, 3_f32],
                                &[0_f32,1_f32, 3_f32]];

        let mat: Matrix<f32> = Matrix::from(data);

        let comp: Vec<f32> = vec![0_f32, 1_f32, 9_f32, 1_f32];
        
        assert_eq!(
                    mat.get_col(1),
                    comp 
                );

    }

    #[test]
    fn test_mult() {
        let data1: &[&[f32]] = &[
                                    &[1_f32, 2_f32],
                                    &[3_f32, 4_f32],
                                    &[5_f32, 6_f32],
                                ];
        let data2: &[&[f32]] = &[
                                    &[1_f32, 2_f32, 3_f32],
                                    &[4_f32, 5_f32, 6_f32]
                                ];

        let mat1: Matrix<f32> = Matrix::from(data1);
        let mat2: Matrix<f32> = Matrix::from(data2);

        let comp: Vec<f32> = vec![9_f32, 12_f32, 15_f32,
                                     19_f32,26_f32, 33_f32,
                                     29_f32,40_f32, 51_f32];

        assert_eq!(
                (mat1 * mat2).data,
                comp);
    }

    #[test]
    fn test_transpose() {
        let data1: &[&[f32]] = &[
                                    &[1_f32, 2_f32],
                                    &[3_f32, 4_f32],
                                    &[5_f32, 6_f32],
                                ];
        
        let mut mat1: Matrix<f32> = Matrix::from(data1);

        let comp: Vec<f32> = vec![1_f32, 3_f32, 5_f32,
                                2_f32, 4_f32, 6_f32];


        assert_eq!(mat1.transpose().data, comp);

    }

    #[test]
    fn test_exp() {

        let data1: &[&[f32]] = &[
                                    &[1_f32, 2_f32],
                                    &[3_f32, 4_f32],
                                    &[5_f32, 6_f32],
                                ];

        let mut mat1: Matrix<f32> = Matrix::from(data1);

        let comp: Vec<f32> = vec![1_f32.exp(), 2_f32.exp(), 3_f32.exp(),
                                    4_f32.exp(), 5_f32.exp(), 6_f32.exp()];

        assert_eq!(mat1.exp().data, comp);
    }

    #[test]
    fn test_mult_scalar() {
        let data1: &[&[f32]] = &[
                                    &[1_f32, 2_f32],
                                    &[3_f32, 4_f32],
                                    &[5_f32, 6_f32],
                                ];

        let mat1: Matrix<f32> = Matrix::from(data1);

        let comp: Vec<f32> = vec![1_f32 * 5_f32, 2_f32 * 5_f32, 3_f32 * 5_f32,
                                    4_f32 * 5_f32, 5_f32 * 5_f32, 6_f32 * 5_f32];

        assert_eq!((mat1 * 5_f32).data, comp);

    }

    #[test]
    fn test_div_scalar() {
        let data1: &[&[f32]] = &[
                                    &[1_f32, 2_f32],
                                    &[3_f32, 4_f32],
                                    &[5_f32, 6_f32],
                                ];

        let mat1: Matrix<f32> = Matrix::from(data1);

        let comp: Vec<f32> = vec![1_f32 / 5_f32, 2_f32 / 5_f32, 3_f32 / 5_f32,
                                    4_f32 / 5_f32, 5_f32 / 5_f32, 6_f32 / 5_f32];

        assert_eq!((mat1 / 5_f32).data, comp);

    }

    #[test]
    fn test_one_over() {
        let data1: &[&[f32]] = &[
                                    &[1_f32, 2_f32],
                                    &[3_f32, 4_f32],
                                    &[5_f32, 6_f32],
                                ];

        let mat1: Matrix<f32> = Matrix::from(data1);

        let comp: Vec<f32> = vec![1_f32 / 1_f32, 1_f32/2_f32, 
                                  1_f32/  3_f32, 1_f32/4_f32,
                                  1_f32/  5_f32, 1_f32/6_f32];

        assert_eq!(mat1.one_over().data, comp);

    }

}
