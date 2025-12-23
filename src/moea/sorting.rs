//! 非支配ソート

use super::dominance::{find_first_front, remove_individuals};
use crate::types::Population;

/// 非支配ソート
///
/// 結果は以下の形式
/// ```
/// [[front1], [front2], [front3], ...]
/// ```
pub fn non_dominated_sort(population: &Population) -> Vec<Population> {
    if population.is_empty() {
        return vec![];
    }

    let front: Population = find_first_front(population);
    let remaining: Population = remove_individuals(population, &front);
    // HACK: chainを使うよりもmutにしてextendする方が可読性が高いのでこちらを使用
    // ```
    // std::iter::once(front)
    //     .chain(non_dominated_sort(&remaining).into_iter())
    //     .collect();
    // ```
    let mut result: Vec<Population> = vec![front];
    result.extend(non_dominated_sort(&remaining));
    result
}
