#[derive(Debug, Clone)]
pub struct Rank {
    /// 非優越ソートのランク
    pub dominance_rank: f64,
    /// クラウディング距離
    pub crowding_distance: f64,
}

impl Default for Rank {
    fn default() -> Self {
        Self {
            dominance_rank: -1.0,
            crowding_distance: 0.0,
        }
    }
}
