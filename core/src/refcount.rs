use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, PartialOrd, Eq, Ord)]
pub struct RefCount<T> {
    v: Rc<RefCell<T>>,
}

impl<T> RefCount<T> {
    pub fn new(t: T) -> RefCount<T> {
        RefCount {
            v: Rc::new(RefCell::new(t)),
        }
    }
}

impl<T> RefCount<T> {
    pub fn borrow(&self) -> Ref<T> {
        self.v.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.v.borrow_mut()
    }

    pub fn as_ptr(&self) -> *mut T {
        self.v.as_ptr()
    }
}

impl<T: fmt::Display> fmt::Display for RefCount<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl<T: fmt::Debug> fmt::Debug for RefCount<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

//TODO: Look if this is ok...
impl<'a, T> Deref for RefCount<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { self.as_ptr().as_ref().unwrap() }
    }
}
impl<T: PartialEq> PartialEq for RefCount<T> {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<T> Hash for RefCount<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ptr().hash(state)
    }
}
