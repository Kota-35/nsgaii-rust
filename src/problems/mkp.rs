use serde_json::Value;
use std::{
    fs, io,
    io::Error,
    path::{Path, PathBuf},
};

use crate::individual::{Individual, Objectives};

#[derive(Debug)]
pub struct MKP {
    /// 目的関数の数
    pub number_of_obj: usize,

    /// アイテムの数
    pub number_of_items: usize,

    /// ナップザックの容量
    pub capacity: usize,

    /// 各目的関数における各アイテムの利益
    /// ```
    /// profit[obj_index][item_index]
    /// ```
    /// の場合、目的関数(`obj_index`)におけるアイテム(`item_index`)の利益
    pub profit: Vec<Vec<u64>>,

    /// 各アイテムの重さ
    pub weight: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MKPInstance {
    /// 目的関数2, アイテム100, インスタンス1
    P2N100Ins1,
    /// 目的関数2, アイテム20, インスタンス1
    P2N20Ins1,
    /// 目的関数3, アイテム100, インスタンス1,
    P3N100Ins1,
    /// 目的関数4, アイテム40, インスタンス10,
    P4N40Ins10,
    /// 目的関数5, アイテム20, インスタンス10
    P5N20Ins10,
}

impl MKPInstance {
    /// インスタンスに対応するファイルパスを取得
    pub fn file_path(&self) -> PathBuf {
        PathBuf::from("problems/mkp").join(self.filename())
    }

    // インスタンスに対応するファイル名を取得
    pub fn filename(&self) -> &'static str {
        match self {
            Self::P2N20Ins1 => "KP_p-2_n-20_ins-1.json",
            Self::P2N100Ins1 => "KP_p-2_n-100_ins-1.json",
            Self::P3N100Ins1 => "KP_p-3_n-100_ins-1.json",
            Self::P4N40Ins10 => "KP_p-4_n-40_ins-10.json",
            Self::P5N20Ins10 => "KP_p-5_n-20_ins-10.json",
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::P2N100Ins1,
            Self::P2N20Ins1,
            Self::P3N100Ins1,
            Self::P4N40Ins10,
            Self::P5N20Ins10,
        ]
    }
}

impl std::fmt::Display for MKPInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.filename())
    }
}

impl MKP {
    /// インスタンスから問題を読み込む
    pub fn from_instance(instance: MKPInstance) -> Result<Self, Error> {
        Self::from_file(&instance.file_path())
    }

    /// ファイルパスから問題を読み込む
    pub fn from_file(path: &Path) -> Result<Self, Error> {
        fs::read_to_string(path)
            .map_err(|e| {
                Error::new(
                    io::ErrorKind::Other,
                    format!("ファイル読み込みエラー: {}", e),
                )
            })
            .and_then(|content| {
                serde_json::from_str::<Value>(&content).map_err(|e| {
                    Error::new(
                        io::ErrorKind::InvalidData,
                        format!("JSONパースエラー: {}", e),
                    )
                })
            })
            .and_then(|json| {
                Self::from_json_value(&json).ok_or_else(|| {
                    Error::new(
                        io::ErrorKind::InvalidData,
                        "JSON構造が不正です",
                    )
                })
            })
    }

    /// JSONのValueからMKPを構築する（関数型スタイル）
    fn from_json_value(json: &Value) -> Option<Self> {
        json.get("number_of_obj")
            .and_then(|v| v.as_u64())
            .map(|n| n as usize)
            .zip(
                json.get("number_of_items")
                    .and_then(|v| v.as_u64())
                    .map(|n| n as usize),
            )
            .zip(
                json.get("capacity")
                    .and_then(|v| v.as_u64())
                    .map(|n| n as usize),
            )
            .zip(json.get("profit").and_then(|v| v.as_array()).and_then(
                |arr| {
                    arr.iter()
                        .map(|obj| {
                            obj.as_array().and_then(|items| {
                                items
                                    .iter()
                                    .map(|item| item.as_u64())
                                    .collect::<Option<Vec<_>>>()
                            })
                        })
                        .collect::<Option<Vec<_>>>()
                },
            ))
            .zip(json.get("weight").and_then(|v| v.as_array()).and_then(
                |arr| {
                    arr.iter()
                        .map(|w| w.as_u64().map(|n| n as usize))
                        .collect::<Option<Vec<_>>>()
                },
            ))
            .map(
                |(
                    (((number_of_obj, number_of_items), capacity), profit),
                    weight,
                )| {
                    Self {
                        number_of_obj,
                        number_of_items,
                        capacity,
                        profit,
                        weight,
                    }
                },
            )
    }
}

/// 適合度を計算する
pub fn fit_mkp(mkp: &MKP, x: &Individual) -> Objectives {
    // 各目的関数の利益を計算
    let f: Vec<f64> = (0..mkp.number_of_obj)
        .map(|k| {
            (0..x.len())
                .map(|i| mkp.profit[k][i] as f64 * x[i] as f64)
                .sum()
        })
        .collect();

    // 重さの合計を計算
    let total_weight: usize =
        (0..x.len()).map(|i| mkp.weight[i] * x[i] as usize).sum();

    // 容量制約違反のペナルティ
    if total_weight > mkp.capacity {
        // 制約違反時は、すべての目的関数値を -(totalWeight - capacity) に設定
        let penalty = -((total_weight - mkp.capacity) as f64);
        vec![penalty; mkp.number_of_obj]
    } else {
        f
    }
}
