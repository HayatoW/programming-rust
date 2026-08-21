mod ref_with_flag {
    use std::marker::PhantomData;

    /// `&T` と `bool` を1ワードに押し込める。
    pub struct RefWithFlag<'a, T> {
        ptr_and_bit: usize,
        behaves_like: PhantomData<&'a T>, // メモリは占有しない
    }
}
