use std::ptr;

pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> Vec<T> {
    pub fn push(&mut self, elem: T) {

        if self.len == self.cap {
            self.reallocate();
        } unsafe {
            ptr::write(self.ptr.add(self.len), elem);
            self.len += 1;
        }
    }
}