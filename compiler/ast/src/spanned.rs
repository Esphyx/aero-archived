pub struct Spanned<T> {
    pub node: T,
    pub position: usize,
    pub length: usize,
}
