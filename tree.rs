use std::fmt::{Show, Formatter, Result};

#[deriving(Eq, Clone)]
pub enum Tree<T> {
    Leaf(T),
    Branch(~[Tree<T>])
}

impl<T> Tree<T> {
    pub fn is_leaf(&self) -> bool {
        match *self {
            Leaf(_) => true,
            _       => false
        }
    }

    pub fn is_branch(&self) -> bool {
        !self.is_leaf()
    }

    pub fn get_ref_leaf<'a>(&'a self) -> &'a T {
        match *self {
            Leaf(ref val) => val,
            _         => fail!("called Tree<T>::get_ref_leaf() on Branch()"),
        }
    }

    pub fn get_ref_branch<'a>(&'a self) -> &'a ~[Tree<T>] {
        match *self {
            Branch(ref val) => val,
            _         => fail!("called Tree<T>::get_ref_branch() on Leaf()"),
        }
    }

    pub fn unwrap_leaf(self) -> T {
        match self {
            Leaf(val) => val,
            _         => fail!("called Tree<T>::unwrap_leaf() on Branch()"),
        }
    }

    pub fn unwrap_branch(self) -> ~[Tree<T>] {
        match self {
            Branch(val) => val,
            _         => fail!("called Tree<T>::unwrap_branch() on Leaf()"),
        }
    }

}

impl<T: Show> Show for Tree<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Branch(ref vec) => {
                write!(f.buf, "({})", *vec)
            },
            Leaf(ref val) => write!(f.buf, "{}", *val)
        }
    }
}

impl<T: Show> Show for ~[Tree<T>] {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut vec_iter = self.iter();
        let mut s = format!("{}", *vec_iter.next().unwrap());

        for e in vec_iter {
            s = s + ~" " + format!("{}", *e);
        }

        write!(f.buf, "{}", s)
    }
}
