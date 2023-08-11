use std::ptr:: NonNull;         
use std::alloc::{self, Layout};
use std::mem;





#[derive(Debug)]
 pub struct Storage {

    data: *mut u8,  
    capacity: usize 
   
}


impl<T> Storage<T>{

    pub const fn new() -> Self {
        Self { ptr: NonNull::<T>::dangling(), capacity: 0 }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self::allocate(capacity)
    }

    pub unsafe fn from_raw_parts(ptr: *mut T, capacity: usize) -> Self {
        Self { ptr: unsafe { NonNull::new_unchecked(ptr) }, capacity: capacity }
    }


    pub fn copy_from(&self, source : *const T, size: usize){
        let dest_pointer = self.ptr.as_ptr()
        unsafe {std::ptr::copy(source, dest_pointer, size)};




    }
  
    pub fn copy_to(&self) -> Self{
        
        let layout = Layout::array::<T>(self.cap_bytes).unwrap();
        let result = unsafe{alloc::alloc(layout) };

        let copy_inst = Storage {
                ptr: unsafe { NonNull::new_unchecked(result as *mut T) },
                cap_bytes: self.capacity,
            }

        }



    }
    fn allocate(capacity: usize) -> Self {
      
        let layout = Layout::array::<T>(capacity).unwrap();
        let result = unsafe{alloc::alloc(layout) };
            
        Self {
                ptr: unsafe { NonNull::new_unchecked(result as *mut T) },
                cap_bytes: capacity,
            }
        }
    
    pub fn grow_exact(&mut self, unit: usize) {
        self.capacity += unit;
        let new_layout = Layout::array::<T>(self.capacity).unwrap();
        let new_ptr =  unsafe{alloc::alloc(new_layout)};
            
        self.ptr = match NonNull::new(new_ptr as *mut T) {
                                    Some(p) => p,
                                    None => alloc::handle_alloc_error(new_layout),
                                };  

    }

  
    pub fn grow_mul(&mut self, mul: usize) {
        self.cap_bytes *= mul;
        let new_layout = Layout::array::<T>(self.cap_bytes).unwrap();
        let new_ptr = if self.cap_bytes==0 {
            unsafe{alloc::alloc(new_layout)}
                                    } else {unsafe{alloc::alloc(new_layout)}};
                
        self.ptr = match NonNull::new(new_ptr as *mut T) {
                                        Some(p) => p,
                                        None => alloc::handle_alloc_error(new_layout),
                                    };  
    
    }


    pub fn grow(&mut self){
        let (new_cap, new_layout) = if self.cap_bytes ==0{ (1, Layout::array::<T>(1).unwrap())} else {
            let new_c = 2 * self.cap_bytes;
            let new_l = Layout::array::<T>(new_c).unwrap();
            (new_c, new_l)

        };
        
        let new_ptr = if self.cap_bytes==0 {
            unsafe{alloc::alloc(new_layout)}
                                    } else {unsafe{alloc::alloc(new_layout)}};
                
        self.ptr = match NonNull::new(new_ptr as *mut T) {
                                        Some(p) => p,
                                        None => alloc::handle_alloc_error(new_layout),
                                    };                           
        self.cap_bytes = new_cap;
    }


    

    pub fn capacity(&self) -> usize {
         self.cap_bytes 
    }

    pub fn data_ptr(&self) -> *mut u8 {
        self.data
    }

  
}

