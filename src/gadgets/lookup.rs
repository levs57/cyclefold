// This implements a log-up lookup protocol.
// Few fixes that we need to eventually apply:
// 1. LOG-UP benefits greatly from the fact that a lot of values in it are zero.
// We are currently unable to exploit it.
// 2. Table right now is implemented as priveleged subset of variables. Considering it is the same for all
// step instances, it is not, actually, getting folded. This should be made a primitive.

use std::{iter::repeat_with, rc::Rc};

use ff::{PrimeField, BatchInvert};
use itertools::Itertools;
use num_bigint::BigUint;

use crate::{constraint_system::Variable, utils::field_precomp::FieldUtils, circuit::{Build, Circuit, ExternalValue, Advice}, gate::Gatebb, gadgets::lc::sum_gadget};

/// Outputs a product of vector elements and products skipping a single element.
pub fn montgomery<F: PrimeField+FieldUtils>(v: &[F]) -> (F, Vec<F>) {
    if v.len() == 0 {return (F::ONE, vec![])}

    let mut left = vec![F::ONE];
    let mut right = vec![F::ONE];

    let l = v.len();
    for i in 0..l-1 {
        let last = left[i];
        left.push(last*v[i]);
    }
    for i in 0..l-1 {
        let last = right[i];
        right.push(last*v[l-i-1]);
    }

    let prod = left[l-1]*v[l-1];

    let mut ret = vec![];

    for i in 0..l-1 {
        ret.push(left[i]*right[l-i-1])
    }

    (prod, ret)
}

/// Parses input as a, c, vals[0], ... vals[k-1] and constrains a = \sum 1/(vals[i]-c)
/// Assumes denominators are nonzero (which always happens provided c is a random challenge); unsound otherwise.
pub fn sum_of_fractions<'a, F:PrimeField+FieldUtils> (args: &[F], k: usize) -> F {
    let (tmp, vals) = args.split_at(2);
    assert_eq!(vals.len(), k);
    let (res, c) = (tmp[0], tmp[1]);
    let (prod, skips) = montgomery(& vals.iter().map(|t|*t-c).collect_vec());
    res*prod-skips.iter().fold(F::ZERO, |acc,upd|acc+upd)
}

pub fn sum_of_fractions_with_nums<'a, F:PrimeField+FieldUtils> (args: &[F], k: usize) -> F {
    let (tmp1, tmp2) = args.split_at(2);
    assert!(tmp2.len() == 2*k);
    let (vals, nums) = tmp2.split_at(k);
    let (res, c) = (tmp1[0], tmp1[1]);
    let (prod, skips) = montgomery(& vals.iter().map(|t|*t-c).collect_vec());
    res * prod - skips.iter().zip_eq(nums.iter()).fold(F::ZERO, |acc,(skip, num)|acc + *skip * num)
}

pub fn invsum_flat_constrain<'a, F: PrimeField+FieldUtils>(
    circuit: &mut Circuit<'a, F, Gatebb<'a, F>, Build>,
    vals: &[Variable],
    res: Variable,
    challenge: Variable,
    ) -> (){
        assert!(vals.len()>0);
        let args = [res, challenge].iter().chain(vals.iter()).map(|x|*x).collect_vec();
        let k = vals.len();
        let gate = Gatebb::new(vals.len()+1, args.len(), 1, Rc::new(move |args|vec![sum_of_fractions(args, k)]));
        circuit.constrain(&args, gate);        
    }

pub fn fracsum_flat_constrain<'a, F: PrimeField+FieldUtils>(
    circuit: &mut Circuit<'a, F, Gatebb<'a, F>, Build>,
    vals: &[Variable],
    nums: &[Variable],
    res: Variable,
    challenge: Variable,
) -> () {
    assert!(vals.len()==nums.len());
    let args = [res, challenge].iter().chain(vals.iter()).chain(nums.iter()).map(|x|*x).collect_vec();
    let k = vals.len();
    let gate = Gatebb::new(vals.len()+1, args.len(), 1, Rc::new(move |args|vec![sum_of_fractions_with_nums(args, k)]));
    circuit.constrain(&args, gate);
}

/// Gadget which returns the sum of inverses of an array, shifted by a challenge.
/// Assumes that array length is divisible by rate, pad otherwise.
/// Unsound if one of the inverses is undefined.
/// Rate - amount of values processed in a batch. Deg = rate+1
pub fn invsum_gadget<'a, F: PrimeField+FieldUtils>(
    circuit: &mut Circuit<'a, F, Gatebb<'a, F>, Build>,
    vals: &[Variable],
    challenge: Variable,
    rate: usize,
    round: usize,
    ) -> Variable {
        assert!(rate > 0);
        let l = vals.len();
        assert!(l%rate == 0);
        let mut vals = vals;
        let mut chunk;
        let advice = Advice::new(l+1, 0, l/rate, move |args: &[F], _|{
            let (args, c) = args.split_at(l);
            let c = c[0];
            let mut inv = args.iter().map(|x|*x-c).collect_vec();
            inv.batch_invert();
            let mut ret = vec![];
            let mut inv : &[F] = &inv;
            let mut chunk;
            while inv.len() > 0 {
                (chunk, inv) = inv.split_at(rate);
                ret.push(chunk.iter().fold(F::ZERO, |acc, upd|{acc+upd}));
            }
            ret
        });

        let mut args = vals.to_vec();
        args.push(challenge);

        let batches = circuit.advice(round, advice, args, vec![]);
        for i in 0..l/rate {
            (chunk, vals) = vals.split_at(rate);
            invsum_flat_constrain(circuit, chunk, batches[i], challenge);
        }

    sum_gadget(circuit, &batches, round)        

    }



/// 
pub trait Lookup<'a, F: PrimeField+FieldUtils> {
    /// Adds the variable to the list of variables to look up.
    fn check(&mut self, circuit: &mut Circuit<'a, F, Gatebb<'a,F>, Build>, var: Variable) -> ();
    /// Seals the lookup and applies the constraints. Returns the challenge.
    /// Round parameter is the round of a challenge - so it must be strictly larger than rounds of any
    /// variable participating in a lookup.
    fn finalize(
        self,
        circuit: &mut Circuit<'a, F, Gatebb<'a,F>, Build>,
        table_round: usize,
        access_round: usize,
        challenge_round: usize
    ) -> ExternalValue<F>;
}

pub struct RangeLookup<F: PrimeField+FieldUtils> {
    vars: Vec<Variable>,
    round: usize,
    challenge: ExternalValue<F>,
    range: usize,
}

impl<'a, F: PrimeField+FieldUtils> RangeLookup<F> {
    pub fn new(range: usize) -> Self {
        
        Self{
            vars: vec![],
            round: 0,
            challenge: ExternalValue::<F>::new(),
            range,
        }
    }
}

impl<'a, F: PrimeField+FieldUtils> Lookup<'a, F> for RangeLookup<F> {
    fn check(&mut self, _circuit: &mut Circuit<'a, F, Gatebb<'a,F>, Build>, var: Variable) -> () {
        if self.round < var.round {
            self.round = var.round
        }
        self.vars.push(var);
    }
    fn finalize(
        self,
        circuit: &mut Circuit<'a, F, Gatebb<'a,F>, Build>,
        table_round: usize,
        access_round: usize,
        challenge_round: usize
    ) -> ExternalValue<F> {
        let Self{vars, round, challenge, range} = self;

        assert!(table_round <= access_round);
        assert!(access_round >= round);
        assert!(challenge_round > access_round);

        // Table of values 0, 1, ..., range-1
        let read_table = Advice::new(0, 0, range, move |_:&[F], _| {
            (0..range).map(|i|F::from(i as u64)).collect()
        });
        let _table = circuit.advice(table_round, read_table, vec![], vec![]);
        // Access counts.
        let compute_accesses = Advice::new(vars.len(), 0, range, move |vars: &[F], _|{
            let mut ret = vec![0; range];
            for var in vars{
                let var = BigUint::from_bytes_le(var.to_repr().as_ref());
                assert!(var < range.into(), "Error: lookup value out of range.");
                let i = var.to_u64_digits()[0] as usize;
                ret[i]+=1;
            }
            ret.into_iter().map(|x|F::from(x)).collect()
        });
        let _access_counts = circuit.advice(access_round, compute_accesses, vars, vec![]);
        // Allocate challenge.
        todo!("CONSTRAIN STUFF");

        challenge
    }
}