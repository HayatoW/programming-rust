//! 検証用の書き捨て

use std::collections::BTreeMap;

fn main() {
    // 都市とそこにある公園のテーブル。それぞれの値はベクタ
    let mut parks = BTreeMap::new();

    parks.insert("Portland", vec!["Mt. Tabor Park", "Forest Park"]);
    parks.insert("Kyoto", vec!["Tadasu-no-Mori Forest", "Maruyama Koen"]);
    parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);

    // すべての公園からなるベクタを作る。`values` は個々のベクタを返すイテレータを作る。
    // `flatten` で各ベクタの要素をアイテムとして生成するイテレータを作る。
    // 要素型のみ推論させる。
    let all_parks: Vec<_> = parks.values().flatten().cloned().collect();

    assert_eq!(
        all_parks,
        vec![
            "Tadasu-no-Mori Forest",
            "Maruyama Koen",
            "Percy Warner Park",
            "Dragon Park",
            "Mt. Tabor Park",
            "Forest Park"
        ]
    );
}
