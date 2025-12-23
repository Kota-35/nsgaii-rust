//! 非支配ソート

use super::dominance::{find_first_front, remove_individuals};
use crate::types::Individual;

/// 非支配ソート
///
/// 結果は以下の形式
/// ```
/// [[front1], [front2], [front3], ...]
/// ```
pub fn non_dominated_sort(
    population: &Vec<Individual>,
) -> Vec<Vec<Individual>> {
    if population.is_empty() {
        return vec![];
    }

    let front: Vec<Individual> = find_first_front(population);
    let remaining: Vec<Individual> =
        remove_individuals(population, &front);
    // HACK: chainを使うよりもmutにしてextendする方が可読性が高いのでこちらを使用
    // ```
    // std::iter::once(front)
    //     .chain(non_dominated_sort(&remaining).into_iter())
    //     .collect();
    // ```
    let mut result: Vec<Vec<Individual>> = vec![front];
    result.extend(non_dominated_sort(&remaining));
    result
}

/// フロント数を指定して非支配ソートを実行s
pub fn non_dominated_sort_n(
    n: usize,
    population: &Vec<Individual>,
) -> Vec<Vec<Individual>> {
    if n == 0 || population.is_empty() {
        Vec::new()
    } else {
        let front: Vec<Individual> = find_first_front(population);
        let remaining: Vec<Individual> =
            remove_individuals(population, &front);

        let mut result: Vec<Vec<Individual>> = vec![front];

        result.append(&mut non_dominated_sort_n(n - 1, &remaining));

        result
    }
}

/// 各個体にフロント番号をつける
pub fn assign_front_ranks(
    population: &Vec<Individual>,
) -> Vec<(Individual, usize)> {
    let fronts = non_dominated_sort(population);

    fronts
        .iter()
        .enumerate()
        .flat_map(|(idx, front)| {
            let rank = idx + 1;
            front
                .iter()
                .map(move |individual| (individual.clone(), rank))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_dominated_sort_n_zero() {
        // n=0の場合、空のベクトルを返す
        let population = vec![vec![1, 2, 3], vec![2, 1, 3]];
        let result = non_dominated_sort_n(0, &population);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_non_dominated_sort_n_empty_population() {
        // 空の集団の場合、空のベクトルを返す
        let population = vec![];
        let result = non_dominated_sort_n(3, &population);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_non_dominated_sort_n_one() {
        // n=1の場合、第1フロントのみを返す
        let population = vec![
            vec![1, 5, 5], // 第1フロント
            vec![5, 1, 5], // 第1フロント
            vec![3, 3, 3], // 第2フロント（vec![2, 2, 2]に支配される）
            vec![2, 2, 2], // 第1フロント
        ];
        let result = non_dominated_sort_n(1, &population);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
        assert!(result[0].contains(&vec![1, 5, 5]));
        assert!(result[0].contains(&vec![5, 1, 5]));
        assert!(result[0].contains(&vec![2, 2, 2]));
    }

    #[test]
    fn test_non_dominated_sort_n_two() {
        // n=2の場合、第1と第2フロントを返す
        let population = vec![
            vec![1, 1, 1], // 第1フロント（最良）
            vec![2, 2, 2], // 第2フロント（vec![1, 1, 1]に支配される）
            vec![3, 3, 3], // 第3フロント（vec![2, 2, 2]に支配される）
            vec![4, 4, 4], // 第4フロント（vec![3, 3, 3]に支配される）
        ];
        let result = non_dominated_sort_n(2, &population);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0], vec![1, 1, 1]);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[1][0], vec![2, 2, 2]);
    }

    #[test]
    fn test_non_dominated_sort_n_more_than_fronts() {
        // nが実際のフロント数より大きい場合、すべてのフロントを返す
        let population = vec![
            vec![1, 1, 1], // 第1フロント
            vec![2, 2, 2], // 第2フロント
            vec![3, 3, 3], // 第3フロント
        ];
        let result = non_dominated_sort_n(5, &population);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0][0], vec![1, 1, 1]);
        assert_eq!(result[1][0], vec![2, 2, 2]);
        assert_eq!(result[2][0], vec![3, 3, 3]);
    }

    #[test]
    fn test_non_dominated_sort_n_less_than_fronts() {
        // nが実際のフロント数より小さい場合、指定された数だけのフロントを返す
        let population = vec![
            vec![1, 1, 1], // 第1フロント
            vec![2, 2, 2], // 第2フロント
            vec![3, 3, 3], // 第3フロント
            vec![4, 4, 4], // 第4フロント
        ];
        let result = non_dominated_sort_n(2, &population);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0][0], vec![1, 1, 1]);
        assert_eq!(result[1][0], vec![2, 2, 2]);
    }

    #[test]
    fn test_non_dominated_sort_n_multiple_non_dominated() {
        // 複数の非支配個体が存在する場合
        let population = vec![
            vec![1, 5, 5], // 第1フロント
            vec![5, 1, 5], // 第1フロント（互いに支配関係なし）
            vec![5, 5, 1], // 第1フロント（互いに支配関係なし）
            vec![2, 2, 2], // 第1フロント（互いに支配関係なし）
            vec![3, 3, 3], // 第2フロント（vec![2, 2, 2]に支配される）
        ];
        let result = non_dominated_sort_n(2, &population);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 4);
        assert!(result[0].contains(&vec![1, 5, 5]));
        assert!(result[0].contains(&vec![5, 1, 5]));
        assert!(result[0].contains(&vec![5, 5, 1]));
        assert!(result[0].contains(&vec![2, 2, 2]));
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[1][0], vec![3, 3, 3]);
    }

    #[test]
    fn test_non_dominated_sort_n_single_individual() {
        // 単一の個体の場合
        let population = vec![vec![5, 5, 5]];
        let result = non_dominated_sort_n(1, &population);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0], vec![5, 5, 5]);
    }

    #[test]
    fn test_non_dominated_sort_n_same_values() {
        // すべて同じ値の場合（すべてが第1フロント）
        let population = vec![vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5]];
        let result = non_dominated_sort_n(1, &population);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
    }
}
