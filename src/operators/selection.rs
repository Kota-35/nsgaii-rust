// //! 選択に関する処理

// use crate::types::Population;

// /// NSGA-II のバイナリトーナメント選択を行う。
// ///
// /// - 2 個体をランダムに抽出し、支配ランクの低い方を選択。
// /// - ランクが同じ場合はクラウディング距離が大きい方を選択。
// /// - 上記を `n` 回繰り返して新しい `Population` を構成する。
// pub fn binary_tournament_nsga2(
//     population: &Population,
//     n: usize,
// ) -> Population {
//     assert!(n > 2, "トーナメント回数は 2 以上でなけらばならない");
//     // 2個体をランク優先（同ランク時はクラウディング距離）で選ぶバイナリトーナメント
//     (0..n)
//         .map(|_| {
//             let i = rand::random_range(0..population.len());
//             let j = (1 + rand::random_range(1..population.len()))
//                 % population.len();

//             let ind1 = &population[i];
//             let ind2 = &population[j];

//             match (ind1.rank.dominance_rank, ind2.rank.dominance_rank) {
//                 // ランクの小さい方を集める
//                 (r1, r2) if r1 < r2 => ind1.clone(),
//                 (r1, r2) if r1 > r2 => ind2.clone(),
//                 _ => {
//                     // 同じランクの場合はクラウディング距離で比較
//                     match (
//                         ind1.rank.crowding_distance,
//                         ind2.rank.crowding_distance,
//                     ) {
//                         (cd1, cd2) if cd1 > cd2 => ind1.clone(),
//                         _ => ind2.clone(),
//                     }
//                 }
//             }
//         })
//         .collect()
// }
