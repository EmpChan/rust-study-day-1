fn main(){
    // rust by example
    // primitives
    let logical: bool = true;
    
    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    let mut mutable = 12;
    mutable = 21;

    //mutable = true; <- this is error

    let my_array: [i32; 5] = [1,2,3,4,5];

    let my_tuple = (5u32,1u8,true,-5.04f32);
}