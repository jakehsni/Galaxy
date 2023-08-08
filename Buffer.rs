use std::ptr:: NonNull;
use std::alloc::{self, Layout};
use std::mem;


struct allocator;

#[derive(Debug)]
struct Tensor<T>{
    data : *mut T, 
    cap_bytes : usize,
    strides : Vec<usize>,
    dimensions :  Vec<usize>,
    ndim : usize,
    size : usize

}


impl<T> Tensor<T>{

   
    pub fn with_capacity(capacity: usize) -> Self {
        Self::allocate(capacity)
    }

    pub fn from_raw_parts(ptr: *mut T, capacity: usize) -> Self {
        Self { ptr: unsafe { NonNull::new_unchecked(ptr) }, cap_bytes: capacity }
    }


    pub fn receive_data(&self, source : *const T, size: usize){
        let dest_pointer = self.ptr.as_ptr();
        unsafe {std::ptr::copy(source, dest_pointer, size)};




    }
   
   


    
    fn allocate(capacity: usize) -> Self {
      
        let layout = Layout::array::<T>(capacity).unwrap();
        let result = unsafe{alloc::alloc(layout) };
            
        Self {
                ptr: result as *mut T },
                cap_bytes: capacity,
            }
        }
    
  fn get_stride_from_shape(new_shape : Vec<usize>, ndim : usize) -> Vec<usize>{


let mut v: Vec<usize> = Vec::new();

for i in 0..new_shape.len(){
    v.push()
}



  }


        fn view(&self, new_shape : Vec<usize>) -> Self {


            Self{
                data : self.data,
                size : self.size,
            }



        }
   
 

    pub fn raw_mut_ptr(&self) -> *mut T {
        self.data
    }

    pub fn raw_const_ptr(&self) -> *const T {
        self.data as *const T
    }

}
