/*!
One-line description.

More detailed description, with

# Example

*/

use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct RcRefCell<T: Sized> {
    inner: Rc<RefCell<T>>,
}

#[derive(Debug)]
pub struct WeakRefCell<T: Sized> {
    inner: Weak<RefCell<T>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<T> Clone for RcRefCell<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl<T> RcRefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(value)),
        }
    }

    pub fn as_inner(&self) -> &Rc<RefCell<T>> {
        &self.inner
    }

    pub fn unwrap(self) -> T {
        match Rc::try_unwrap(self.inner) {
            Ok(ref_cell) => ref_cell.into_inner(),
            _ => panic!("could not unwrap the std::rc::Rc value"),
        }
    }

    pub fn downgrade(self) -> WeakRefCell<T> {
        WeakRefCell {
            inner: Rc::downgrade(&self.inner),
        }
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}

// ------------------------------------------------------------------------------------------------

impl<T> Clone for WeakRefCell<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl<T> WeakRefCell<T> {
    pub fn as_inner(&self) -> &Weak<RefCell<T>> {
        &self.inner
    }

    pub fn upgrade(self) -> Option<RcRefCell<T>> {
        match self.inner.upgrade() {
            None => None,
            Some(inner) => Some(RcRefCell { inner }),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct Node {
        pub name: String,
        pub parent: Option<NodeRef>,
    }

    type NodeRef = RcRefCell<Node>;

    impl Node {
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                parent: None,
            }
        }
    }

    #[test]
    fn test_ref_aliasing() {
        let node = Node::new("name-1");
        let ref1: NodeRef = RcRefCell::new(node);
        {
            assert_eq!(ref1.borrow().name, "name-1");
            assert!(ref1.borrow().parent.is_none());
            let inner = ref1.borrow();
            assert_eq!(inner.name, "name-1");
            assert!(inner.parent.is_none());
        }
        let ref2: NodeRef = ref1.clone();
        {
            assert_eq!(ref2.borrow().name, "name-1");
            assert!(ref2.borrow().parent.is_none());
        }

        {
            let mut mut_inner = ref2.borrow_mut();
            mut_inner.parent = Some(ref2.clone());
            assert!(mut_inner.parent.is_some());
        }
        {
            assert_eq!(
                ref2.borrow().parent.as_ref().unwrap().borrow().name,
                "name-1"
            );
        }

        {
            let mut mut_inner = ref2.borrow_mut();
            mut_inner.name = "name-2".to_string();
            assert_eq!(mut_inner.name, "name-2");
        }

        {
            let ref2 = ref2.borrow();
            assert_eq!(ref2.name, ref2.parent.as_ref().unwrap().borrow().name);
        }
        assert_eq!(ref2.borrow().name, "name-2");
        assert_eq!(ref1.borrow().name, "name-2");
    }
}
