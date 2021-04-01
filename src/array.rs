/// A single element of an array.
///
/// In a more classical algorithmic sense, a potential leaf node in a tree.
#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Array(Array),
    Int(i64), // TODO: f64, bigint, etc
    String(String),
    None,
}

impl Element {
    pub fn as_array(&self) -> Option<&Array> {
        match self {
            Element::Array(array) => Some(array),
            _ => None,
        }
    }
}

/// An array.
///
/// The term "array" is misleading here, because elements can themselves be arrays. The more accurate word to describe
/// the basic data structure of Sponk, and indeed most array-oriented programming languages, is tree.
#[derive(Debug, PartialEq, Clone)]
pub struct Array {
    shape: Vec<usize>,
    data: Vec<Element>,
}

impl Array {
    pub fn new() -> Array {
        Array {
            shape: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn from_element(element: Element) -> Array {
        Array {
            shape: vec![1],
            data: vec![element],
        }
    }

    pub fn relegate(&self) -> Option<Element> {
        if self.is_scalar() {
            Some(self.data[0].clone())
        } else {
            None
        }
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn is_scalar(&self) -> bool {
        self.shape.len() == 1 && self.shape[0] == 1
    }

    pub fn for_each_monad<F>(&self, f: F) -> Array
    where
        F: Fn(&Element) -> Element,
    {
        let data = self.data.iter().map(f).collect();

        Array {
            shape: self.shape.clone(),
            data,
        }
    }

    /// Do ⍺f⍵
    ///
    /// Only under the following circumstances:
    ///
    /// (⍴⍺ = ⍴⍵) ∨ (⍴⍺ = 1) ∨ (⍴⍵ = 1)
    pub fn for_each_dyad<F>(&self, f: F, array: &Array) -> Array
    where
        F: Fn(&Element, &Element) -> Element,
    {
        assert!(self.shape == array.shape || self.is_scalar() || array.is_scalar());

        if self.shape == array.shape {
            let data = self
                .data
                .iter()
                .zip(array.data.iter())
                .map(|(my, their)| f(my, their))
                .collect();

            Array {
                shape: self.shape.clone(),
                data,
            }
        } else if array.shape[0] == 1 {
            let data = self.data.iter().map(|my| f(my, &array.data[0])).collect();

            Array {
                shape: self.shape.clone(),
                data,
            }
        } else if self.shape[0] == 1 {
            let data = array
                .data
                .iter()
                .map(|their| f(&self.data[0], their))
                .collect();

            Array {
                shape: array.shape.clone(),
                data,
            }
        } else {
            unreachable!("shape unsound");
        }
    }

    /// The / operator
    pub fn fold<F>(&self, f: F) -> Element
    where
        F: Fn(Element, &Element) -> Element,
    {
        // TODO this isn't exactly correct, this is only supposed to operate on the top rank
        self.data
            .iter()
            .skip(1)
            .fold(self.data[0].clone(), |acc, next| f(acc, next))
    }

    // monad ,
    pub fn ravel(&self) -> Array {
        Array {
            shape: vec![self.data.len()],
            data: self.data.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn array1() {
        let array = Array {
            shape: vec![2, 4],
            data: vec![
                Element::Int(1),
                Element::Int(2),
                Element::Int(3),
                Element::Int(4),
                Element::Int(5),
                Element::Int(6),
                Element::Int(7),
                Element::Int(8),
            ],
        };

        let result = array.for_each_monad(|element| match element {
            Element::Int(i) => Element::Int(i * 3),
            _ => Element::None,
        });

        assert_eq!(
            result,
            Array {
                shape: vec![2, 4],
                data: vec![
                    Element::Int(1 * 3),
                    Element::Int(2 * 3),
                    Element::Int(3 * 3),
                    Element::Int(4 * 3),
                    Element::Int(5 * 3),
                    Element::Int(6 * 3),
                    Element::Int(7 * 3),
                    Element::Int(8 * 3),
                ],
            }
        );
    }

    #[test]
    fn array2() {
        let array = Array {
            shape: vec![2, 4],
            data: vec![
                Element::Int(1),
                Element::Int(2),
                Element::Int(3),
                Element::Int(4),
                Element::Int(5),
                Element::Int(6),
                Element::Int(7),
                Element::Int(8),
            ],
        };

        let out = array.for_each_dyad(
            |l, r| match l {
                Element::Int(l) => match r {
                    Element::Int(r) => Element::Int(l + r),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            &array,
        );

        assert_eq!(
            out,
            Array {
                shape: vec![2, 4],
                data: vec![
                    Element::Int(1 + 1),
                    Element::Int(2 + 2),
                    Element::Int(3 + 3),
                    Element::Int(4 + 4),
                    Element::Int(5 + 5),
                    Element::Int(6 + 6),
                    Element::Int(7 + 7),
                    Element::Int(8 + 8),
                ],
            }
        );
    }

    #[test]
    fn array3() {
        let array2 = Array {
            shape: vec![1],
            data: vec![Element::Int(4)],
        };

        assert!(array2.is_scalar());
    }

    #[test]
    fn array4() {
        let array = Array {
            shape: vec![2, 4],
            data: vec![
                Element::Int(1),
                Element::Int(2),
                Element::Int(3),
                Element::Int(4),
                Element::Int(5),
                Element::Int(6),
                Element::Int(7),
                Element::Int(8),
            ],
        };

        let array2 = Array {
            shape: vec![1],
            data: vec![Element::Int(4)],
        };

        let out = array.for_each_dyad(
            |l, r| match l {
                Element::Int(l) => match r {
                    Element::Int(r) => Element::Int(l + r),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            &array2,
        );

        assert_eq!(
            out,
            Array {
                shape: vec![2, 4],
                data: vec![
                    Element::Int(1 + 4),
                    Element::Int(2 + 4),
                    Element::Int(3 + 4),
                    Element::Int(4 + 4),
                    Element::Int(5 + 4),
                    Element::Int(6 + 4),
                    Element::Int(7 + 4),
                    Element::Int(8 + 4),
                ],
            }
        );
    }

    #[test]
    fn array5() {
        let array = Array {
            shape: vec![2, 4],
            data: vec![
                Element::Int(1),
                Element::Int(2),
                Element::Int(3),
                Element::Int(4),
                Element::Int(5),
                Element::Int(6),
                Element::Int(7),
                Element::Int(8),
            ],
        };

        let result = array.fold(|acc, next| match acc {
            Element::Int(acc) => match next {
                Element::Int(next) => Element::Int(acc + next),
                _ => panic!(),
            },
            _ => panic!(),
        });

        assert_eq!(result, Element::Int(1 + 2 + 3 + 4 + 5 + 6 + 7 + 8));
    }
}
