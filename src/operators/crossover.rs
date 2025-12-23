//! 交叉に関する処理

use crate::individual::Individual;

/// 1点交叉を行う。
///
/// 遺伝子が交叉する場所（交叉点）をランダムで一つ選び、その場所より後ろを入れ換える方式である。
/// ホランドが最初に提案したときの交叉方法であるが、効率は低く現在ではあまり使われていない。
/// ```
/// 個体A: 01001｜11010 ⇒ 01001 01011
/// 個体B: 10101｜01011 ⇒ 10101 11010
/// ```
fn one_point_crossover(
    parent1: &Individual,
    parent2: &Individual,
    point: usize,
) -> (Individual, Individual) {
    let (p1_left, p1_right) = parent1.variables.split_at(point);
    let (p2_left, p2_right) = parent2.variables.split_at(point);

    return (
        Individual::new([p1_left, p2_right].concat()),
        Individual::new([p2_left, p1_right].concat()),
    );
}

/// ランダムな点で1点交叉を行う
///
/// 遺伝子が交叉する場所（交叉点）をランダムで一つ選び、その場所より後ろを入れ換える方式である。
/// ホランドが最初に提案したときの交叉方法であるが、効率は低く現在ではあまり使われていない。
/// ```
/// 個体A: 01001｜11010 ⇒ 01001 01011
/// 個体B: 10101｜01011 ⇒ 10101 11010
/// ```
pub fn one_point_crossover_random(
    parent1: &Individual,
    parent2: &Individual,
) -> (Individual, Individual) {
    let len = parent1.variables.len().min(parent2.variables.len());
    let point = rand::random_range(0..len);

    return one_point_crossover(parent1, parent2, point);
}

#[cfg(test)]
mod tests {
    use super::one_point_crossover;
    use crate::individual::Individual;

    fn create_parent() -> (Individual, Individual) {
        return (
            Individual::new(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            Individual::new(vec![9, 8, 7, 6, 5, 4, 3, 2]),
        );
    }

    #[test]
    fn test_one_point_crossover_at_start() {
        let (parent1, parent2) = create_parent();
        let (child1, child2) = one_point_crossover(&parent1, &parent2, 0);

        // 交叉点が0の場合、すべて入れ替わる
        assert_eq!(child1.variables, vec![9, 8, 7, 6, 5, 4, 3, 2]);
        assert_eq!(child2.variables, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_one_point_crossover_at_middle() {
        let (parent1, parent2) = create_parent();
        let (child1, child2) = one_point_crossover(&parent1, &parent2, 4);

        // 交叉点が4の場合
        // parent1: [1,2,3,4 | 5,6,7,8]
        // parent2: [9,8,7,6 | 5,4,3,2]
        // child1: [1,2,3,4,5,4,3,2]
        // child2: [9,8,7,6,5,6,7,8]
        assert_eq!(child1.variables, vec![1, 2, 3, 4, 5, 4, 3, 2]);
        assert_eq!(child2.variables, vec![9, 8, 7, 6, 5, 6, 7, 8]);
    }

    #[test]
    fn test_one_point_crossover_at_end() {
        let (parent1, parent2) = create_parent();
        let (child1, child2) = one_point_crossover(&parent1, &parent2, 8);

        // 交叉点が最後の場合、入れ替わらない
        assert_eq!(child1.variables, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(child2.variables, vec![9, 8, 7, 6, 5, 4, 3, 2]);
    }

    #[test]
    fn test_one_point_crossover_different_lengths() {
        let parent1 = Individual::new(vec![1, 2, 3]);
        let parent2 = Individual::new(vec![4, 5, 6, 7, 8]);
        let (child1, child2) = one_point_crossover(&parent1, &parent2, 2);

        // parent1: [1,2 | 3]
        // parent2: [4,5 | 6,7,8]
        // child1: [1,2,6,7,8]
        // child2: [4,5,3]
        assert_eq!(child1.variables, vec![1, 2, 6, 7, 8]);
        assert_eq!(child2.variables, vec![4, 5, 3]);
    }
}
