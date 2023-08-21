use std::vec;

#[derive(Debug)]
pub struct Matris<T>{
    data:Vec<Vec<T>>,
    shape:(usize,usize),
}
// pub union  Type{
//     I8:i8,
//     I16:i16,
//     I32:i32,
//     I64:i64,
//     I128:i128,
//     U8:u8,
//     U16:u16,
//     U32:u32,
//     U64:u64,
//     U128:u128,
//     F32:f32,
//     F64:f64,
//     USIZE:usize,
//     ISIZE:isize,
// }

impl<T:std::clone::Clone +std::fmt::Debug> Matris<T> {
    pub fn data(&self)->Vec<Vec<T>>{
        self.data.clone()
    }
    pub fn shape(&self)->(usize,usize){
        self.shape.clone()
    }
    pub fn vec_tranform(d:Vec<Vec<T>>)->Matris<T>{   // MATRIX  must contain same no of element in each column
        let x= d.len();
        let y=d[0].len();
        d.iter().map(|x|{
            if x.len()!=y{panic!("PROVIDED SHAPE  OF MATRIX IS INCORRECT")}
            x
        });
        Matris { data: d, shape: (0,9) }
    }
    pub fn gen_numericals(value:T,shape:(usize,usize))->Matris<T>{
        if shape.0<1 || shape.1<1{panic!("PROVIDED SHAPE {:?} IS IN CORRECT </ SHAPE MUST BE GREATER_EQ THEN (1,1)/>",shape)}
        Matris{
            data:vec![vec![value;shape.1];shape.0],
            shape:shape
        }
    }
    pub fn zeros(shape:(usize,usize))->Matris<f64>{
        Matris::gen_numericals(0.0 as f64, shape)
    }
    pub fn ones(shape:(usize,usize))->Matris<f64>{
        Matris::gen_numericals(1.0 as f64, shape)
    }
    pub fn squeeze(&self)->Vec<T>{
        let mut v=Vec::with_capacity(self.shape.0*self.shape.1+2);
        for i in self.data.clone().into_iter(){
            v.extend(i)
        }
        v
    }
    pub fn reshape(&mut self,shape:(usize,usize),inplace:bool)->Matris<T>{
        if self.shape.0*self.shape.1!=shape.0*shape.1{
            panic!("ENTERED SHAPE IS INVALID")
        }
        let mut v=Vec::with_capacity(shape.0*shape.1+2);
        let (mut start_in,mut end_in)=(0 as usize,shape.1);
        let mut v1=Vec::<T>::with_capacity(shape.0+2);
        for (i,val) in self.squeeze().into_iter().enumerate(){
            v1.push(val);
            if (i+1)%shape.1 == 0{
                v.push(v1.clone());
                v1.clear();
            }
            
        }
        Matris{data:v,shape:shape}
    }
}