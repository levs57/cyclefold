use group::{Group, Curve};
use ff::{Field, PrimeField};
use halo2::{arithmetic::{best_multiexp, best_fft}};
use halo2curves::CurveAffine;
use num_traits::pow;

/// A simple commitment key.
pub enum CkS<G: CurveAffine>{
    Trivial,
    Group(Vec<G>),
}

/// Commitment target.
pub enum Ct<G: CurveAffine>{
    Trivial(Vec<G::Scalar>),
    Group(G),
}

/// Commitment key trait.
pub trait Ck<G: CurveAffine> {
    fn commit(&self, wtns: &Vec<G::Scalar>) -> Ct<G>;
} 

impl<G: CurveAffine> Ck<G> for CkS<G>{
    fn commit(&self, wtns: &Vec<G::Scalar>) ->  Ct<G>{
        match self {
            CkS::Trivial => Ct::Trivial(wtns.clone()),
            CkS::Group(v) => Ct::Group(best_multiexp(wtns, v).to_affine()),
        }
    }
}

pub trait RootsOfUnity where Self : PrimeField{
    /// Returns power of a primitive root of unity of order 2^logorder.
    fn roots_of_unity(power: u64, logorder: usize) -> Self;
    /// Returns power of 1/2.
    fn half_pow(power: u64) -> Self;
}

/// A generic black-box gate. This API is unsafe, you must guarantee that given value is a
/// homogeneous polynomial of degree d with i inputs and o outputs. 
pub struct Gatebb<'a, F : PrimeField> {
    d : usize,
    i : usize,
    o : usize,
    f : Box<dyn Fn(&Vec<F>) -> Vec<F> + 'a>,
}

impl<'a, F: PrimeField> Gatebb<'a, F> {
    pub fn new(d: usize, i: usize, o: usize, f: Box<dyn Fn(&Vec<F>) -> Vec<F> + 'a>) -> Self {
        Gatebb::<'a>{d,i,o,f}
    }
}

pub trait Gate<F : PrimeField> {
    /// Returns degree.
    fn d(&self) -> usize;
    /// Returns input size.
    fn i(&self) -> usize;
    /// Returns output size.
    fn o(&self) -> usize;
    /// Executes gate on a given input. Must ensure the correct length of an input.
    fn exec(&self, input : &Vec<F>) -> Vec<F>;
    /// Returns coefficients of  f(in1 + x in2) in x (for example, 0-th is f(in1) and d-th is f(in2))
    fn cross_terms(&self, in1: &Vec<F>, in2: &Vec<F>) -> Vec<Vec<F>>;
}

impl<'a, F : PrimeField + RootsOfUnity> Gate<F> for Gatebb<'a, F> {
    /// Returns degree.
    fn d(&self) -> usize {
        self.d
    }
    /// Returns input size.
    fn i(&self) -> usize{
        self.i
    }
    /// Returns output size.
    fn o(&self) -> usize{
        self.o
    }
    /// Executes gate on a given input. Must ensure the correct length of an input.
    fn exec(&self, input : &Vec<F>) -> Vec<F>{
        assert!(input.len() == self.i);
        let tmp = (self.f)(input);
        assert!(tmp.len() == self.o);
        tmp
    }
    /// Returns coefficients of  f(in1 + x in2) in x (for example, 0-th is f(in1) and d-th is f(in2))
    fn cross_terms(&self, in1: &Vec<F>, in2: &Vec<F>) -> Vec<Vec<F>> {
        let mut d = self.d;
        if d == 0 {
            return vec![self.exec(in1)]
        }
        
        let mut logorder = 0;
        while d>0 {
            d>>=1;
            logorder +=1;
        }

        let mut values = vec![vec![]; self.o];

        let omega_inv = F::roots_of_unity(pow(2, logorder)-1, logorder);
        let scale = F::half_pow(logorder as u64);

        for i in 0..pow(2, logorder){
            let t = F::roots_of_unity(i, logorder);
            let tmp = self.exec(&in1.iter().zip(in2.iter()).map(|(x,y)| (*x + *y * t)).collect());
            for j in 0..self.o {
                values[j].push(tmp[j]);
            }
        }

        values.iter_mut().map(|v| {
            best_fft(v, omega_inv, logorder as u32);
            v.iter_mut().map(|x|*x *= scale).count();
        }).count();

        let mut ret = vec![vec![]; (self.d+1)];
        for i in 0..(self.d+1) {
            for j in 0..self.o {
                ret[i].push(values[j][i])
            }
        }

        assert!({
            let mut flag = true;
            for i in (self.d+1)..pow(2,logorder) {
                for j in 0..self.o{
                    flag &= (values[j][i] == F::ZERO)
                }
            }
            flag
        }, "fft failed");

        ret

    }
}