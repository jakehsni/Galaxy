use std::alloc::{self, Layout};

mod storage;
use storage::Storage;


#[derive(Debug)]
 pub struct Buffer {
    buf: Storage,
    len : usize
}


impl<T> Buffer<T>{
    

    pub fn replace_from_vector(&mut self, vector: &Vec<>) {
        let len = vector.len();
        if (len >= self.buf.capacity ){
            self.buf.grow_exact(len)
        }
        self.buf.copy_from(vector.as_ptr(), len);
        self.len = len;
    }


    pub fn take_vector(&mut self, vector: &Vec<>) {
        self.len = vector.len();
        self.buf.from_raw_parts(vector.as_ptr(), self.len);
    }


    pub fn replace_from_slice(&mut self, slice: &[T]) {
        self.len = slice.len();
        self.buf.copy_from(slice.as_ptr(), self.len);
    }


    pub fn copy_to(&self) -> Self {
        let buf  = self.buf.copy_to();
        let len = self.len();
        std::ptr::copy(self.data_ptr(), buf.data_ptr(), len);
        Self{buf: buf,
        len : len}
    }


    pub fn release(&mut self) {
        self.len = 0;
    }
    

    pub fn len(&self) -> usize {
        self.len
    }


    pub fn data_ptr(&self) -> *mut T {
        self.buf.data_ptr()
    }
  
}

