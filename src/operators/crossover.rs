//! 交叉に関する処理

use rand::RngCore;

use crate::individual::Individual;

/// 1点交叉を行う。
///
/// `ind1` と `ind2` の変数長が一致していることを前提に、ランダムな交叉点で
/// 前半を親1→子1/親2→子2、後半を親2→子1/親1→子2として入れ替える。
/// 戻り値は生成された2個体と、交叉点以降で値が異なった変数数を示す。
/// # Panics
/// 親個体の変数長が一致しない場合にパニックする。
pub fn crossover_1p(
    ind1: &Individual,
    ind2: &Individual,
) -> (Individual, Individual, usize) {
    assert_eq!(
        ind1.variables.len(),
        ind2.variables.len(),
        "親個体の変数長が一致していません"
    );

    let len = ind1.variables.len();
    let k = rand::random_range(0..len);

    let (vars1, vars2, nvar_change) = ind1
        .variables
        .iter()
        .zip(ind2.variables.iter())
        .enumerate()
        .fold(
            (Vec::with_capacity(len), Vec::with_capacity(len), 0usize),
            |(mut c1, mut c2, mut diff), (i, (a, b))| {
                if i < k {
                    c1.push(*a); // 交叉点より前は親1->子1, 親2->子2
                    c2.push(*b);
                } else {
                    c1.push(*b);
                    c2.push(*a);
                    if a != b {
                        diff += 1;
                    }
                }
                (c1, c2, diff)
            },
        );

    (Individual::new(vars1), Individual::new(vars2), nvar_change)
}
