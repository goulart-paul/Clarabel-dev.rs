#![allow(non_snake_case)]
use crate::algebra::{DenseFactorizationError, DenseMatrix, FloatT, Matrix};

pub trait FactorEigen {
    type T: FloatT;
    // computes eigenvalues only (full set)
    fn eigvals(&mut self, A: &mut Matrix<Self::T>) -> Result<(), DenseFactorizationError>;
    // computes eigenvalues and vectors (full set)
    fn eigen(&mut self, A: &mut Matrix<Self::T>) -> Result<(), DenseFactorizationError>;
}

pub trait FactorCholesky {
    type T: FloatT;
    // computes the Cholesky decomposition.  Only the upper
    // part of the input A will be referenced, and A will
    // by modified in place.   The Cholesky factor is stored
    // in self.L
    fn cholesky(&mut self, A: &mut Matrix<Self::T>) -> Result<(), DenseFactorizationError>;
}

pub trait FactorSVD {
    type T: FloatT;
    // compute "economy size" SVD.  Values in A are overwritten
    // as internal working space.
    fn svd(&mut self, A: &mut Matrix<Self::T>) -> Result<(), DenseFactorizationError>;
}

pub trait MultiplySYRK {
    type T: FloatT;
    fn syrk<MATA>(&mut self, A: &MATA, α: Self::T, β: Self::T) -> &Self
    where
        MATA: DenseMatrix<T = Self::T>;
}

pub trait MultiplyGEMM {
    type T: FloatT;
    fn mul<MATA, MATB>(&mut self, A: &MATA, B: &MATB, α: Self::T, β: Self::T) -> &Self
    where
        MATB: DenseMatrix<T = Self::T>,
        MATA: DenseMatrix<T = Self::T>;
}

pub trait MultiplyGEMV {
    type T: FloatT;
    fn gemv(&self, x: &[Self::T], y: &mut [Self::T], α: Self::T, β: Self::T);
}

pub trait MultiplySYMV {
    type T: FloatT;
    fn symv(&self, x: &[Self::T], y: &mut [Self::T], α: Self::T, β: Self::T);
}
