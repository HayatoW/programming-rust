mod gap {
    use std::ops::Range;

    pub struct GapBuffer<T> {
        /// 要素を格納する場所。必要とする容量を持つが、長さは常にゼロになる
        /// GapBuffer は要素とギャップを `Vec` が「使用していない領域」に置く
        storage: Vec<T>,

        /// `storage` 内で初期化されていない要素の範囲を指す
        /// この範囲の前後の領域は常に初期化されている
        gap: Range<usize>,
    }
}
