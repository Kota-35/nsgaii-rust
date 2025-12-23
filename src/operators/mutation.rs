use crate::types::Individual;

/// ビット反転突然変異

/// ビット(0 or 1)を反転する
fn flip_bit(bit: &u8) -> u8 {
    // XORを使用
    // A | B | Y
    // 0 | 0 | 0
    // 0 | 1 | 1
    // 1 | 0 | 1
    // 1 | 1 | 0
    *bit ^ 1
}

/// 指定された確率でビットを反転する
fn mutate_bit(prob: f64, bit: &u8) -> u8 {
    let r: f64 = rand::random_range(0.0..1.0);
    if r < prob { flip_bit(bit) } else { bit.clone() }
}

/// Individualに突然変異を適応する
fn bit_flip_mutation(prob: f64, ind: &Individual) -> Vec<u8> {
    ind.iter().map(|bit| mutate_bit(prob, bit)).collect()
}

// /// 複数の個体に突然変異を適用する
// fn mutate_population
