#[cfg(test)]
mod tests {
    use std::{rc::Rc, iter::repeat};

    use crate::{gate::Gatebb, constraint_system::Variable, circuit::{Circuit, ExternalValue, PolyOp, Advice}, gadgets::{poseidon::{poseidon_gadget, Poseidon, poseidon_gadget_mixstrat}, bits::bit_decomposition_gadget, bit_chunks::bit_chunks_gadget, ecmul::{add_proj}}};
    use ff::{PrimeField, Field};
    use group::{Group, Curve};
    use halo2curves::{bn256, grumpkin, CurveAffine, CurveExt};
    use rand_core::OsRng;
    use crate::utils::poly_utils::{check_poly, find_degree};
    use crate::utils::field_precomp::FieldUtils;
    
    type F = bn256::Fr;
    type C = grumpkin::G1;
    
    // #[test]
    
    // fn test_cross_terms() {
    
    //     for d in 2..10{
    //         let f = Rc::new(|v: &[F]| vec![v[0].pow([2 as u64])]);
    //         let gate = Gatebb::new(2, 1, 1, f);
    //         let tmp = gate.cross_terms_adjust(&vec![F::ONE], &vec![F::ONE], d);
    //         println!("{:?}", tmp.iter().map(|v|v[0]).collect::<Vec<_>>());
    //     }
    // }
    
    #[test]
    
    fn test_circuit_builder() {
        let public_input_source = ExternalValue::<F>::new();
    
        let mut circuit = Circuit::<F, Gatebb<F>>::new(2, 1);
    
        let sq = PolyOp::new(2, 1, 1, Rc::new(|x|vec!(x[0]*x[0])));
        let input = circuit.advice_pub(0, Advice::new(0, 1, 1, Rc::new(|_, iext|vec![iext[0]])), vec![], vec![&public_input_source])[0];
        let sq1 = circuit.apply(0, sq.clone(), vec![input]);
        let _ = circuit.apply_pub(0, sq.clone(), sq1);
    
        circuit.finalize();
    
        public_input_source.set(F::from(2)).unwrap();
    
        circuit.execute(0);
    
        println!("{:?}", circuit.cs.getvar(Variable::Public(0,2)).to_repr());
    }
    
    #[test]
    
    fn test_permutation_argument() {
        let pi_ext : Vec<_> = repeat(ExternalValue::<F>::new()).take(5).collect();
        let challenge_ext = ExternalValue::<F>::new();
    
        let mut circuit = Circuit::<F, Gatebb<F>>::new(2, 2);
        
        let one = Variable::Public(0,0);
    
        let read_pi_advice = Advice::new(0,1,1, Rc::new(|_, iext: &[F]| vec![iext[0]]));
        
    
        let mut pi = vec![];
        for k in 0..5{
            pi.push(
                circuit.advice_pub(0, read_pi_advice.clone(), vec![], vec![&pi_ext[k]])[0]
            );
        }
    
        let challenge = circuit.advice_pub(1, read_pi_advice.clone(), vec![], vec![&challenge_ext])[0];
    
        let division_advice = Advice::new(2, 0, 1, Rc::new(|ivar : &[F], _| {
            let ch = ivar[0];
            let x = ivar[1];
            vec![(x-ch).invert().unwrap()]
        }));
    
        let mut fractions = vec![];
        for k in 0..5 {
            fractions.push(
                circuit.advice(1, division_advice.clone(), vec![challenge, pi[k]], vec![])[0]
            );
        }
    
        let div_constr = Gatebb::<F>::new(2, 4, 1, Rc::new(|args|{
            let one = args[0];
            let ch = args[1];
            let x = args[2];
            let res = args[3];
            vec![one*one - res * (x-ch)]
        }));
    
        for k in 0..5 {
            circuit.constrain(&[one, challenge, pi[k], fractions[k]], div_constr.clone());
        }
    
        circuit.finalize();
    
        // construction phase ended
    
        pi_ext[0].set(F::from(2)).unwrap();
        pi_ext[1].set(F::from(3)).unwrap();
        pi_ext[2].set(F::from(4)).unwrap();
        pi_ext[3].set(F::from(5)).unwrap();
        pi_ext[4].set(F::from(6)).unwrap();
    
        circuit.execute(0);
    
        challenge_ext.set(F::random(OsRng)).unwrap();
        circuit.execute(1);
    
        circuit.cs.valid_witness(); // test that constraints are satisfied
    }
    
    #[test]
    fn test_poseidon_gadget(){
        let cfg = Poseidon::new();
        let pi_ext = ExternalValue::<F>::new();
        let mut circuit = Circuit::<F, Gatebb<F>>::new(25, 1);
        let read_pi_advice = Advice::new(0,1,1, Rc::new(|_, iext: &[F]| vec![iext[0]]));    
        let pi = circuit.advice_pub(0, read_pi_advice.clone(), vec![], vec![&pi_ext])[0];
        let ret = poseidon_gadget(&mut circuit, &cfg, 1, 0, vec![pi]);
    
        circuit.finalize();
    
        pi_ext.set(F::ONE).unwrap();
    
        circuit.execute(0);
        circuit.cs.valid_witness();
    
        assert_eq!(circuit.cs.getvar(ret), F::from_str_vartime("18586133768512220936620570745912940619677854269274689475585506675881198879027").unwrap());
    }
    
    #[test]
    fn test_poseidon_gadget_k_equals_two(){
        let cfg = Poseidon::new();
        let pi_ext = ExternalValue::<F>::new();
        let mut circuit = Circuit::<F, Gatebb<F>>::new(25, 1);
        let read_pi_advice = Advice::new(0,1,1, Rc::new(|_, iext: &[F]| vec![iext[0]]));    
        let pi = circuit.advice_pub(0, read_pi_advice.clone(), vec![], vec![&pi_ext])[0];
        let ret = poseidon_gadget(&mut circuit, &cfg, 2, 0, vec![pi]);
    
        circuit.finalize();
    
        pi_ext.set(F::ONE).unwrap();
    
        circuit.execute(0);
        circuit.cs.valid_witness();
    
        assert_eq!(circuit.cs.getvar(ret), F::from_str_vartime("18586133768512220936620570745912940619677854269274689475585506675881198879027").unwrap());
    }

    #[test]
    fn test_poseidon_gadget_mixstrat(){
        let cfg = Poseidon::new();
        let pi_ext = ExternalValue::<F>::new();
        let mut circuit = Circuit::<F, Gatebb<F>>::new(25, 1);
        let read_pi_advice = Advice::new(0,1,1, Rc::new(|_, iext: &[F]| vec![iext[0]]));    
        let pi = circuit.advice_pub(0, read_pi_advice.clone(), vec![], vec![&pi_ext])[0];
        let ret = poseidon_gadget_mixstrat(&mut circuit, &cfg, 0, vec![pi]);

        circuit.finalize();

        pi_ext.set(F::ONE).unwrap();

        circuit.execute(0);
        circuit.cs.valid_witness();

        assert!(circuit.cs.getvar(ret) == F::from_str_vartime("18586133768512220936620570745912940619677854269274689475585506675881198879027").unwrap());

        println!("{:?}", circuit.cs.getvar(ret).to_repr());
    }

    #[test]
    
    fn test_bit_decomposition(){
        let pi_ext = ExternalValue::<F>::new();
        let mut circuit = Circuit::<F, Gatebb<F>>::new(2, 1);
        let read_pi_advice = Advice::new(0,1,1, Rc::new(|_, iext: &[F]| vec![iext[0]]));    
        let pi = circuit.advice_pub(0, read_pi_advice.clone(), vec![], vec![&pi_ext])[0];
    
        let bits = bit_decomposition_gadget(&mut circuit, 0, 3, pi);
    
        circuit.finalize();
        pi_ext.set(F::from(6)).unwrap();
        circuit.execute(0);
    
        circuit.cs.valid_witness();
    
        assert!(bits.len()==3);
        assert!(circuit.cs.getvar(bits[0]) == F::ZERO);
        assert!(circuit.cs.getvar(bits[1]) == F::ONE);
        assert!(circuit.cs.getvar(bits[2]) == F::ONE);
    }
    
    #[test]
    
    fn test_chunk_decomposition(){
        let pi_ext = ExternalValue::<F>::new();
        let mut circuit = Circuit::<F, Gatebb<F>>::new(4, 1);
        let read_pi_advice = Advice::new(0,1,1, Rc::new(|_, iext: &[F]| vec![iext[0]]));    
        let pi = circuit.advice_pub(0, read_pi_advice.clone(), vec![], vec![&pi_ext])[0];
    
        let chunks = bit_chunks_gadget(&mut circuit, 0, 2, 2, pi);
    
        circuit.finalize();
        pi_ext.set(F::from(6)).unwrap();
        circuit.execute(0);
    
        circuit.cs.valid_witness();
    
        assert!(chunks.len()==2);
        assert!(circuit.cs.getvar(chunks[0]) == F::from(2));
        assert!(circuit.cs.getvar(chunks[1]) == F::from(1));
    }
    
    #[test]
    
    fn test_check_poly() {
        let f = Rc::new(|x: &[F]|{vec![x[0].pow([5])]});
        check_poly(4, 1, 1, f).unwrap();
    }
    
    // #[test]
    
    // fn test_doubling_degree() {
    //     for k in 1..5 {
    //         let f = Rc::new(|args: &[F]|{let tmp = double_k_times_internal::<F,C>(args[0], args[1], k); vec![tmp.0, tmp.1, tmp.2]});
    //         println!("at k={}, deg={}", k, find_degree(1000, 2, 3, f).unwrap());
    //     }
    // }
    
    // #[test]
    
    // fn test_double_k_times() {
    //     let pi_x_ext = ExternalValue::<F>::new();
    //     let pi_y_ext = ExternalValue::<F>::new();
    //     let mut circuit = Circuit::<F, Gatebb<F>>::new(37, 1);
    //     let read_pi_advice = Advice::new(0,1,1, Rc::new(|_, iext: &[F]| vec![iext[0]]));    
    
    //     let x = circuit.advice_pub(0, read_pi_advice.clone(), vec![], vec![&pi_x_ext])[0];
    //     let y = circuit.advice_pub(0, read_pi_advice, vec![], vec![&pi_y_ext])[0];
        
    //     let pt = EcAffinePoint::<F, C>::new(&mut circuit, x,y);
    
    //     let ret = double_k_times_gadget(&mut circuit, 2, 0, pt);
    
    //     circuit.finalize();
    
    //     let randpt = C::random(OsRng).to_affine();
    //     let randx = randpt.x;
    //     let randy = randpt.y;
    
    //     pi_x_ext.set(randx).unwrap();
    //     pi_y_ext.set(randy).unwrap();
    
    //     circuit.execute(0);
    //     circuit.cs.valid_witness();
    
    //     let retx = circuit.cs.getvar(ret.x);
    //     let rety = circuit.cs.getvar(ret.y);
    
    //     let randptproj : C = randpt.into();
    //     let quad = randptproj.double().double().to_affine();
    
    //     assert!(grumpkin::G1Affine::from_xy(retx, rety).unwrap() == quad);
    // }
    
    #[test]
    
    fn test_scale(){
        let x = F::random(OsRng);
        for y in 0..100 {
            assert!(x.scale(y) == x*F::from(y));
        }
    }
    
    #[test]
    
    fn test_add() {
        let pt1 = C::random(OsRng).to_affine();
        let pt2 = C::random(OsRng).to_affine();
    
        let r1 = F::random(OsRng);
        let r2 = F::random(OsRng);
    
        let pt1_ = (pt1.x*r1, pt1.y*r1, r1);
        let pt2_ = (pt2.x*r2, pt2.y*r2, r2);
    
        let pt3_ = add_proj::<F,C>(pt1_, pt2_);
    
        let r3_inv = pt3_.2.invert().unwrap();
        let pt3 = grumpkin::G1Affine::from_xy(pt3_.0*r3_inv, pt3_.1*r3_inv).unwrap();
    
        assert!(Into::<C>::into(pt3) == pt1+pt2);
    }
}