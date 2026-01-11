
#[cfg(test)]
mod generated_marint_tests {
    use marint::sign::MSgn;
    use marint::sign::MSgn::*;
    use marint::marint::MarInt;

    #[derive(Clone, Debug)]
    struct Case {
        a_sign: i8,
        a_limbs: &'static [u64],
        b_sign: i8,
        b_limbs: &'static [u64],
        op: char,
        c_sign: i8,
        c_limbs: &'static [u64],
    }

    fn sgn_from_i8(x: i8) -> MSgn {
        match x {
            -1 => MNeg,
             0 => MZero,
             1 => MPos,
             _ => panic!("invalid sign: {}", x),
        }
    }

    // Normalize to canonical representation:
    // - trim high zero limbs (little-endian => trim from the end)
    // - if limbs become empty => sign must be zero
    fn normalize(mut x: MarInt) -> MarInt {
        while x.limbs.last().copied() == Some(0) {
            x.limbs.pop();
        }
        if x.limbs.is_empty() {
            x.sign = MZero;
        }
        x
    }

    fn mk(sign: i8, limbs: &'static [u64]) -> MarInt {
        normalize(MarInt {
            sign: sgn_from_i8(sign),
            limbs: limbs.to_vec(),
        })
    }

    const CASES: &[Case] = &[
        Case { a_sign: 1, a_limbs: &[1467603958867007173u64, 880839817652935825u64, 10889016403872727659u64], b_sign: 1, b_limbs: &[15332621223028728339u64, 15241491894576404149u64], op: '*', c_sign: 1, c_limbs: &[11897537989619899039u64, 4181239160232342466u64, 13069489231862887859u64, 8645193851461804912u64, 8996972831431544220u64] },
        Case { a_sign: -1, a_limbs: &[14782120271727931364u64], b_sign: 1, b_limbs: &[11362751999683767719u64], op: '+', c_sign: -1, c_limbs: &[3419368272044163645u64] },
        Case { a_sign: -1, a_limbs: &[10726550531079820351u64, 945902206214339516u64], b_sign: -1, b_limbs: &[6527210346600750007u64], op: '-', c_sign: -1, c_limbs: &[4199340184479070344u64, 945902206214339516u64] },
        Case { a_sign: 1, a_limbs: &[2941064563030781345u64, 1237537561146483563u64, 3265208201242350702u64], b_sign: 1, b_limbs: &[16067831647608760656u64, 4207388515188232968u64], op: '*', c_sign: 1, c_limbs: &[10869655267122400080u64, 5624895856507598870u64, 16965797921055764535u64, 297666943191126941u64, 744738444394910999u64] },
        Case { a_sign: -1, a_limbs: &[4067187878524421490u64], b_sign: -1, b_limbs: &[4584273163013613011u64, 15740281559977334202u64, 275353880793836707u64], op: '*', c_sign: 1, c_limbs: &[6203206001252202230u64, 13466520573462738768u64, 395736792000966745u64, 60710766181521672u64] },
        Case { a_sign: 1, a_limbs: &[2354301631874474647u64, 2064760067911202634u64], b_sign: 1, b_limbs: &[4899748858696421489u64, 14570856845884425041u64, 826399271895045115u64, 7391240841832029545u64], op: '-', c_sign: -1, c_limbs: &[2545447226821946842u64, 12506096777973222407u64, 826399271895045115u64, 7391240841832029545u64] },
        Case { a_sign: 1, a_limbs: &[5908734567316290649u64], b_sign: -1, b_limbs: &[9412103494515962900u64, 5432620949560968286u64], op: '*', c_sign: -1, c_limbs: &[17750450759009684212u64, 9055941178947096936u64, 1740139889594207681u64] },
        Case { a_sign: -1, a_limbs: &[11442578575747253666u64, 18232822876059147696u64, 14465122348117232450u64], b_sign: -1, b_limbs: &[9246750962922067394u64, 2593522488299253176u64, 13725487849067330580u64, 18390130280669432086u64], op: '-', c_sign: 1, c_limbs: &[16250916460884365344u64, 2807443685949657095u64, 17707109574659649745u64, 18390130280669432085u64] },
        Case { a_sign: 1, a_limbs: &[5475888709881085614u64, 629886063385153460u64, 10816483443090353225u64, 15837738644090534826u64, 18206001269932105344u64], b_sign: -1, b_limbs: &[16951230168737351896u64, 3553041057466050447u64, 15455827495674658603u64, 11514898880236847307u64, 8455794663923290329u64], op: '*', c_sign: -1, c_limbs: &[5885030542917456592u64, 12469934921383278542u64, 7939875594314394635u64, 1501332261806163254u64, 15627868592771747343u64, 12358891328414162528u64, 17773887655819434256u64, 18360319047578053229u64, 16520178346115098576u64, 8345440679099458048u64] },
        Case { a_sign: -1, a_limbs: &[205915446597571662u64, 9171479825954018780u64], b_sign: -1, b_limbs: &[391202029882265826u64, 1200031220377149506u64, 5487362824180048214u64], op: '-', c_sign: 1, c_limbs: &[185286583284694164u64, 10475295468132682342u64, 5487362824180048213u64] },
    ];

    #[test]
    fn marint_random_cases() {
        for (i, tc) in CASES.iter().enumerate() {
            let a = mk(tc.a_sign, tc.a_limbs);
            let b = mk(tc.b_sign, tc.b_limbs);
            let expected = mk(tc.c_sign, tc.c_limbs);

            // If your ops are implemented for &MarInt instead of owned MarInt,
            // change these lines to: &a + &b, &a - &b, &a * &b
            let got = match tc.op {
                '+' => normalize(a.clone() + b.clone()),
                '-' => normalize(a.clone() - b.clone()),
                '*' => normalize(a.clone() * b.clone()),
                _ => panic!("unknown op: {:?}", tc.op),
            };

            assert_eq!(
                got.sign as i8, expected.sign as i8,
                "case {} sign mismatch: op={} a={:?} b={:?} got={:?} expected={:?}",
                i, tc.op, a, b, got, expected
            );
            assert_eq!(
                got.limbs, expected.limbs,
                "case {} limbs mismatch: op={} a={:?} b={:?} got={:?} expected={:?}",
                i, tc.op, a, b, got, expected
            );
        }
    }
}
