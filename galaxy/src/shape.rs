
fn remove_axes1(shape: &[usize], ndim : usize) -> Vec<usize>{

let mut new = Vec::new();
let mut idx_new = 0;
for idim in 0..ndim{
    if (shape[i]!=1){
        new[idx_new] = shape[idim];
        idx_new += 1;
    }
}


 

}

fn stride_from_shape(dimension: &[usize]) -> Vec<usize>{

    let mut strides : Vec<usize> = dimension.iter().rev().scan(1,|s, &x|{*s = *s * x; Some(*s)}).collect();
    strides.reverse();
strides;

}