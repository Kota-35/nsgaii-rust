//! 優越関係判定

use crate::types::{Individual, Population};

/// 支配関係を判定する
/// 個体Aが個体Bを支配する場合true
///
/// 個体Aが個体Bを支配するのは、次の2条件を満たす時である
/// 1. 全ての目的関数で、Aの値がBの値以下 (`A <= B`)
/// 2. 少なくとも1つの目的関数で、Aの値がBの値より小さい (`A < B`)
fn dominates(a: &Individual, b: &Individual) -> bool {
    let (all_le, any_lt) = a.iter().zip(b.iter()).fold(
        (true, false),
        |(all_le, any_lt), (a_elem, b_elem)| {
            (all_le && a_elem <= b_elem, any_lt || a_elem < b_elem)
        },
    );
    all_le && any_lt
}

/// -- 個体aが個体bに支配されているかチェック
pub fn is_dominated_by(a: &Individual, b: &Individual) -> bool {
    dominates(b, a)
}

/// 単一個体に対する支配カウント(何個の個体に支配されているか)
pub fn count_dominators(
    ind: &Individual,
    population: &Population,
) -> usize {
    population
        .iter()
        .filter(|pop_elem| is_dominated_by(ind, *pop_elem))
        .count()
}

/// 第1フロントを見つける(どの個体にも支配されない個体の集合)
pub fn find_first_front(population: &Population) -> Population {
    population
        .iter()
        .filter(|ind| count_dominators(ind, population) == 0usize)
        .cloned()
        .collect()
}

/// 指定されたフロントを除いた残りの個体を返す
pub fn remove_individuals(
    population: &Population,
    front: &Population,
) -> Population {
    population
        .iter()
        .filter(|ind| !front.contains(ind))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dominates_all_objectives_better() {
        // 個体Aがすべての目的関数で個体Bより優れている場合
        let a = vec![1, 2, 3];
        let b = vec![5, 6, 7];
        assert!(dominates(&a, &b));
    }

    #[test]
    fn test_dominates_some_objectives_better() {
        // 個体Aが一部の目的関数で個体Bより優れている場合
        let a = vec![1, 5, 3];
        let b = vec![5, 6, 7];
        assert!(dominates(&a, &b));
    }

    #[test]
    fn test_dominates_not_dominated() {
        // 個体Aが個体Bを支配しない場合（すべての目的関数で劣っている）
        let a = vec![5, 6, 7];
        let b = vec![1, 2, 3];
        assert!(dominates(&a, &b));
    }

    #[test]
    fn test_dominates_same_values() {
        // すべての目的関数で同じ値の場合（支配関係なし）
        let a = vec![5, 6, 7];
        let b = vec![5, 6, 7];
        assert!(dominates(&a, &b));
    }

    #[test]
    fn test_dominates_no_dominance() {
        // 互いに優劣がつかない場合（支配関係なし）
        let a = vec![1, 10, 3];
        let b = vec![10, 1, 3];
        assert!(dominates(&a, &b));
    }

    #[test]
    fn test_dominates_one_objective_better() {
        // 1つの目的関数のみで優れている場合
        let a = vec![1, 5, 5];
        let b = vec![5, 5, 5];
        assert!(dominates(&a, &b));
    }

    #[test]
    fn test_dominates_all_equal_one_less() {
        // すべての目的関数で等しいが、1つだけ小さい場合
        let a = vec![5, 5, 4];
        let b = vec![5, 5, 5];
        assert!(dominates(&a, &b));
    }

    #[test]
    fn test_dominates_single_objective() {
        // 単一目的関数の場合
        let a = vec![1];
        let b = vec![5];
        assert!(dominates(&a, &b));
    }

    #[test]
    fn test_dominates_single_objective_same() {
        // 単一目的関数で同じ値の場合
        let a = vec![5];
        let b = vec![5];
        assert!(dominates(&a, &b));
    }

    #[test]
    fn test_count_dominators_zero() {
        // 支配されていない個体（カウントが0）
        let ind = vec![1, 2, 3];
        let population = vec![
            vec![5, 6, 7],    // indより劣っている
            vec![10, 10, 10], // indより劣っている
        ];
        assert_eq!(count_dominators(&ind, &population), 0);
    }

    #[test]
    fn test_count_dominators_one() {
        // 1つの個体に支配されている
        let ind = vec![5, 5, 5];
        let population = vec![
            vec![1, 1, 1],    // indを支配する
            vec![10, 10, 10], // indより劣っている
        ];
        assert_eq!(count_dominators(&ind, &population), 1);
    }

    #[test]
    fn test_count_dominators_multiple() {
        // 複数の個体に支配されている
        let ind = vec![5, 5, 5];
        let population = vec![
            vec![1, 1, 1],    // indを支配する
            vec![2, 2, 2],    // indを支配する
            vec![3, 3, 3],    // indを支配する
            vec![10, 10, 10], // indより劣っている
        ];
        assert_eq!(count_dominators(&ind, &population), 3);
    }

    #[test]
    fn test_count_dominators_empty_population() {
        // 空の集団の場合
        let ind = vec![5, 5, 5];
        let population = vec![];
        assert_eq!(count_dominators(&ind, &population), 0);
    }

    #[test]
    fn test_count_dominators_single_individual() {
        // 単一の個体の場合（自分自身はカウントしない）
        let ind = vec![5, 5, 5];
        let population = vec![vec![5, 5, 5]];
        assert_eq!(count_dominators(&ind, &population), 0);
    }

    #[test]
    fn test_count_dominators_same_values() {
        // 同じ値の個体は支配関係なし
        let ind = vec![5, 5, 5];
        let population = vec![vec![5, 5, 5], vec![5, 5, 5]];
        assert_eq!(count_dominators(&ind, &population), 0);
    }

    #[test]
    fn test_count_dominators_no_dominance() {
        // 互いに優劣がつかない場合
        let ind = vec![1, 10, 3];
        let population = vec![
            vec![10, 1, 3], // 互いに支配関係なし
            vec![5, 5, 5],  // 互いに支配関係なし
        ];
        assert_eq!(count_dominators(&ind, &population), 0);
    }

    #[test]
    fn test_find_first_front_all_non_dominated() {
        // すべての個体が第1フロント（支配されていない）
        let population = vec![vec![1, 2, 3], vec![2, 1, 3], vec![3, 2, 1]];
        let first_front = find_first_front(&population);
        assert_eq!(first_front.len(), 3);
        assert!(first_front.contains(&vec![1, 2, 3]));
        assert!(first_front.contains(&vec![2, 1, 3]));
        assert!(first_front.contains(&vec![3, 2, 1]));
    }

    #[test]
    fn test_find_first_front_some_dominated() {
        // 一部の個体が第1フロント
        let population = vec![
            vec![1, 5, 5], // 第1フロント
            vec![2, 2, 2], // 第1フロント（互いに支配関係なし）
            vec![3, 3, 3], // vec![2, 2, 2]に支配される
            vec![5, 1, 5], // 第1フロント（互いに支配関係なし）
        ];
        let first_front = find_first_front(&population);
        assert_eq!(first_front.len(), 3);
        assert!(first_front.contains(&vec![1, 5, 5]));
        assert!(first_front.contains(&vec![5, 1, 5]));
        assert!(first_front.contains(&vec![2, 2, 2]));
    }

    #[test]
    fn test_find_first_front_single_front() {
        // 1つの個体のみが第1フロント
        let population = vec![
            vec![1, 1, 1], // 第1フロント（最良）
            vec![2, 2, 2], // 支配される
            vec![3, 3, 3], // 支配される
            vec![4, 4, 4], // 支配される
        ];
        let first_front = find_first_front(&population);
        assert_eq!(first_front.len(), 1);
        assert_eq!(first_front[0], vec![1, 1, 1]);
    }

    #[test]
    fn test_find_first_front_empty_population() {
        // 空の集団の場合
        let population = vec![];
        let first_front = find_first_front(&population);
        assert_eq!(first_front.len(), 0);
    }

    #[test]
    fn test_find_first_front_single_individual() {
        // 単一の個体の場合
        let population = vec![vec![5, 5, 5]];
        let first_front = find_first_front(&population);
        assert_eq!(first_front.len(), 1);
        assert_eq!(first_front[0], vec![5, 5, 5]);
    }

    #[test]
    fn test_find_first_front_same_values() {
        // すべて同じ値の場合（すべてが第1フロント）
        let population = vec![vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5]];
        let first_front = find_first_front(&population);
        assert_eq!(first_front.len(), 3);
    }

    #[test]
    fn test_find_first_front_complex_case() {
        // 複雑なケース：複数の非支配個体
        let population = vec![
            vec![1, 5, 5], // 第1フロント
            vec![5, 1, 5], // 第1フロント（互いに支配関係なし）
            vec![5, 5, 1], // 第1フロント（互いに支配関係なし）
            vec![2, 2, 2], // 第1フロント（互いに支配関係なし）
            vec![3, 3, 3], // vec![2, 2, 2]に支配される
        ];
        let first_front = find_first_front(&population);
        assert_eq!(first_front.len(), 4);
        assert!(first_front.contains(&vec![1, 5, 5]));
        assert!(first_front.contains(&vec![5, 1, 5]));
        assert!(first_front.contains(&vec![5, 5, 1]));
        assert!(first_front.contains(&vec![2, 2, 2]));
    }
}
