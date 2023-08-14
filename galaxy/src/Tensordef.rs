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
    shape :  Vec<usize>,
    viewcnt : usize,
    is_contiguos : bool,
    op : usize,
    orginal: bool,
    offset: usize,
    numel: usize,
    
    /* The number of dimensions */
    ndim : usize
}


impl Tensor{


    fn is_contiguos(&self) -> bool{
        let shape = &self.shape;
        let strides = &self.strides;
        let  check = shape
            .iter()
            .rev()
            .scan(1,|state, x|{let cur_state = *state;*state = *state *  *x;return Some(cur_state)})
            .zip(strides.iter().rev())
            .all(|(x1, &x2)| x1 == x2);

            check
    }


    fn stride_from_shape(shape: &[usize]) -> Vec<usize>{
       
        let  stride: Vec<usize> = shape
            .iter()
            .rev()
            .scan(1,|state, x|{let cur_state = *state;*state = *state *  *x;return Some(cur_state)}).collect();

            stride
        }


    fn view(&mut self, shape : &[usize]){

        let is_contiguos = self::is_contiguos();
        if is_contiguos {
            self.shape  = 
            self.strides  = Self::stride_from_shape(shape);
            tensor{}
        }
    }


    fn tensor_slice(&self, index_slice: &[std::ops::Range<usize>]) -> Tensor {
        let mut ind = 0;
        let mut start_pos = 0;
        let mut strides :Vec<usize> = Vec::new();
        let mut dimensions :Vec<usize> = Vec::new();
        for i in index_slice{

            if (i.start==0 && i.end==0){
                start_pos += self.strides[ind] * i.start;
            dimensions.push(self.dimensions[ind]);}
            else{
            dimensions.push(i.end - i.start);}
            strides.push(self.strides[ind]);
            
            ind+=1; 
        }
            self.viewcnt +=1;
        let new_ptr = unsafe {self.data.add(start_pos)};

        
        tensor{
            data : new_ptr,
            start_pos : start_pos,
            strides : strides,
            dimensions: dimensions,
            ndim: self.dimensions.len(),
            viewcnt : self.viewcnt,
            is_contiguos : true,
            op : usize,
            orginal: false,
            offset: start_pos,
            numel: usize }
            
        }



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
    



   

   

}

\
fn main(){
  
    
   

let mut v = vec![1,2,3];
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
