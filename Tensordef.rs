use std::ptr:: NonNull;
use std::alloc::{self, Layout};
use std::mem;

mod buffer;
mod shape;
mod utility;
use buffer::Buffer;

#[derive(Debug)]
struct Tensor{
    data : Buffer,
    strides : Vec<usize>,
    dimensions :  Vec<usize>,
    viewcnt : usize,
    orginal: bool,
    ndim : usize,
}


impl Tensor{


fn data(&self) -> *mut u8 {
    self.storage.data_ptr()
}
//=======add
fn add_tensor(&self, other : &Tensor) -> Self {
if (self.size != other.size){
    println
}
let ptr = self.data();
let ptr_other = other.data();

for i in 0..slef.dimensions{
   *ptr_out.add(i) (*ptr.add(i))  + (*ptr_other.add(i));

}
Self{strides : self.strides.to_vec(),
    dimensions : self.dimensions.to_vec(),
    size : self.size,
    ndim : self.ndim,

}

}

//=======add_inplace
fn add_inplace_tensor(&mut self, other : &Tensor) -> Self {
    if (self.size != other.size){
        println
    }
    let ptr = self.data();
    let ptr_other = other.data();
    
    for i in 0..slef.dimensions{
       *ptr.add(i) = (*ptr.add(i))  + (*ptr_other.add(i));
    
    }
   
    
    }
    

//=========sub
fn sub_tensor(&self, other : &Tensor) -> Self {
    if (self.size != other.size){
        println
    }
    let ptr = self.data();
    let ptr_other = other.data();
    
    for i in 0..slef.dimensions{
       *ptr_out.add(i)  = (*ptr.add(i))  - (*ptr_other.add(i));
    
    }
    Self{strides : self.strides.to_vec(),
        dimensions : self.dimensions.to_vec(),
        size : self.size,
        ndim : self.ndim,
    
    }
    
}

//=========sub_inplace
fn sub_inplace_tensor(&mut self, other : &Tensor) -> Self {
    if (self.size != other.size){
        println
    }
    let ptr = self.data();
    let ptr_other = other.data();
    
    for i in 0..slef.dimensions{
       *ptr.add(i)  = (*ptr.add(i))  - (*ptr_other.add(i));
    
    }
    Self{strides : self.strides.to_vec(),
        dimensions : self.dimensions.to_vec(),
        size : self.size,
        ndim : self.ndim,
    
    }
    
}
//=======mul
fn mul_tensor(&self, other : &Tensor) -> Self {
    if (self.size != other.size){
        println
    }
    let ptr = self.data();
    let ptr_other = other.data();
    
    for i in 0..slef.dimensions{
       *ptr_out.add(i)  = (*ptr.add(i))  * (*ptr_other.add(i));
    
    }
    Self{strides : self.strides.to_vec(),
        dimensions : self.dimensions.to_vec(),
        size : self.size,
        ndim : self.ndim,
    
    }
    
}

//=======mul_inplace
fn mul_inplace_tensor(&mut self, other : &Tensor) -> Self {
    if (self.size != other.size){
        println
    }
    let ptr = self.data();
    let ptr_other = other.data();
    
    for i in 0..slef.dimensions{
       *ptr.add(i) =  (*ptr.add(i))  * (*ptr_other.add(i));
    
    }
    Self{strides : self.strides.to_vec(),
        dimensions : self.dimensions.to_vec(),
        size : self.size,
        ndim : self.ndim,
    
    }
    
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
