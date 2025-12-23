use crate::individual::rank::Rank;

#[derive(Debug, Clone)]
pub struct Individual {
    /// 0, 1のバイナリ変数
    pub variables: Vec<u8>,

    /// 選択に使用するランク
    pub rank: Rank,
}

impl Individual {
    pub fn new(variables: Vec<u8>) -> Self {
        Self {
            variables,
            rank: Rank::default(),
        }
    }
}
