use std::ptr::NonNull;         
use std::alloc::{self, Layout};
use std::mem;





#[derive(Debug)]
 pub struct Storage<T> {
    /* Pointer to the raw data buffer */
    data: NonNull<T>, 
    capacity: usize 
   
}


impl<T> Storage<T>{

    pub const fn new() -> Self {
        Self { data: NonNull::<T>::dangling(), capacity: 0 }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self::allocate(capacity)
    }

    pub unsafe fn from_raw_parts(ptr: *mut T, capacity: usize) -> Self {
        Self { data: unsafe { NonNull::new_unchecked(ptr) }, capacity: capacity }
    }


    pub fn copy_from(&self, source : *const T, size: usize){
        let dest_pointer = self.data.as_ptr();
        unsafe {std::ptr::copy(source, dest_pointer, size)};




    }
  
    pub fn copy_to(&self) -> Self{
        
        let layout = Layout::array::<T>(self.capacity).unwrap();
        let result = unsafe{alloc::alloc(layout) };

        let copy_inst = Storage {
            data: unsafe { NonNull::new_unchecked(result as *mut T) },
            capacity: self.capacity,
            };
            copy_inst

        }



    
    fn allocate(capacity: usize) -> Self {
      
        let layout = Layout::array::<T>(capacity).unwrap();
        let result = unsafe{alloc::alloc(layout) };
            
        Self {
                data: unsafe { NonNull::new_unchecked(result as *mut T) },
                capacity: capacity,
            }
        }
    
    pub fn grow_exact(&mut self, unit: usize) {
        self.capacity += unit;
        let new_layout = Layout::array::<T>(self.capacity).unwrap();
        let new_ptr =  unsafe{alloc::alloc(new_layout)};
            
        self.data = match NonNull::new(new_ptr as *mut T) {
                                    Some(p) => p,
                                    None => alloc::handle_alloc_error(new_layout),
                                };  

    }

  
    pub fn grow_mul(&mut self, mul: usize) {
        self.capacity *= mul;
        let new_layout = Layout::array::<T>(self.capacity).unwrap();
        let new_ptr = if self.capacity==0 {
            unsafe{alloc::alloc(new_layout)}
                                    } else {unsafe{alloc::alloc(new_layout)}};
                
        self.data = match NonNull::new(new_ptr as *mut T) {
                                        Some(p) => p,
                                        None => alloc::handle_alloc_error(new_layout),
                                    };  
    
    }


    pub fn grow(&mut self){
        let (new_cap, new_layout) = if self.capacity ==0{ (1, Layout::array::<T>(1).unwrap())} else {
            let new_c = 2 * self.capacity;
            let new_l = Layout::array::<T>(new_c).unwrap();
            (new_c, new_l)

        };
        
        let new_ptr = if self.capacity==0 {
            unsafe{alloc::alloc(new_layout)}
                                    } else {unsafe{alloc::alloc(new_layout)}};
                
        self.data = match NonNull::new(new_ptr as *mut T) {
                                        Some(p) => p,
                                        None => alloc::handle_alloc_error(new_layout),
                                    };                           
        self.capacity = new_cap;
    }


    

    pub fn capacity(&self) -> usize {
         self.capacity 
    }

    pub fn data_ptr(&self) -> *mut T {
        self.data.as_ptr()
    }

}

