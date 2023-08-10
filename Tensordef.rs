use std::ptr:: NonNull;
use std::alloc::{self, Layout};
use std::mem;

mod buffer;
mod shape;
mod utility;
use buffer::Storage;

#[derive(Debug)]
struct Tensor{
    storage : Storage,
    strides : Vec<usize>,
    dimensions :  Vec<usize>,
    viewcnt : usize,
    orginal: bool,
    ndim : usize,
    size : usize
}


impl Tensor{


fn data(&self) -> *mut u8 {
    self.storage.data_ptr()
}







   
   fn get(&self) -> *mut usize{
    &self.ndim as *const usize  as *mut usize
}
   

}

fn print_type_of<T>(_: &T) { 
    println!("{}", std::any::type_name::<T>())
     }

fn main(){
  
    
   

let mut v = vec![1,2,3];
print_type_of(&v);
let ptr  = v.as_mut_ptr() ;

let  t  =  Tensor{
    data : v.as_mut_ptr() as *mut u8, 
    cap_bytes : v.capacity(),
    strides : vec![1,2],
    dimensions :  vec![1,2],
    ndim : 1,
    size : 1,
    viewcnt : 0,
    orginal: true

};


v.iter().for_each(|df| println!("{}", df));
//unsafe{*(t.tensor_buffer().add(1)) = 256;}
unsafe{*t.tensor_buffer().add(1) = 1;}
v.iter().for_each(|df| println!("{}", df));






}
