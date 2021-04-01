/// A single element of an array.
///
/// In a more classical algorithmic sense, a potential leaf node in a tree.
#[derive(Debug, PartialEq)]
pub enum Element {
    Array(Array),
    Int(i64), // TODO: f64, bigint, etc
}

/// An array.
///
/// The term "array" is misleading here, because elements can themselves be arrays. The more accurate word to describe
/// the basic data structure of Sponk, and indeed most array-oriented programming languages, is tree.
#[derive(Debug, PartialEq)]
pub struct Array {
    shape: Vec<usize>,
    data: Vec<Element>,
}
