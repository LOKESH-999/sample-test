
#[derive(Debug)]
pub struct Matris<T>{
    data:Vec<Vec<T>>,
    shape:(usize,usize),
}

pub struct I8(i8);
pub struct I16(i16);
pub enum Type{
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    USIZE,
    ISIZE,
}

impl<T:std::clone::Clone> Matris<T> {
    pub fn vec_tranform(d:Vec<Vec<T>>)->Result<Matris<T>,&'static str>{
        Ok(Matris { data: d, shape: (0,9) })
    }

    pub fn gen_numericals(value:T,shape:(usize,usize))->Matris<T>{
        if shape.0<1 || shape.1<1{panic!("PROVIDED SHAPE {:?} IS IN CORRECT </ SHAPE MUST BE GREATER_EQ THEN (1,1)/>",shape)}
        Matris{
            data:vec![vec![value;shape.1];shape.0],
            shape:shape
        }
    }

}

// pub fn zeros<T:std::clone::Clone>(shape:(usize,usize),Type:Type)->Matris<T>{

//     match Type {
//         Type::I8=>{return Matris::gen_numericals(0 as i8, shape);},
//         Type::I16=>{return Matris::gen_numericals(0 as i16, shape);},
//         // Type::I32=>{0.0 as i32},
//         // Type::U128=>{0.0 as i128},
//         _=>{0}
//     };
//     return Matris::gen_numericals(0, shape);
// }