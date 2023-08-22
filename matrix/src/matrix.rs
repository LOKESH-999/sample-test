use std::vec;

// pub fn zeros(shape:(usize,usize))->Matris<f64>{
//     Matris::gen_numericals(0.0, shape)
// }
// pub fn ones(shape:(usize,usize))->Matris<f64>{
//     Matris::gen_numericals(1.0, shape)
// }


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

impl<T:std::clone::Clone +std::fmt::Debug+ std::ops::Mul + std::iter::Sum<<T as std::ops::Mul>::Output>+ std::cmp::PartialOrd+ std::cmp::Ord
+ std::ops::Div<usize, Output = T>+ std::iter::Sum> 
    Matris<T> {

        pub fn data(&self)->Vec<Vec<T>>{
            self.data.clone()
        }
        pub fn shape(&self)->(usize,usize){
            self.shape.clone()
        }
        pub fn from_vec(d:Vec<Vec<T>>)->Matris<T>{   // MATRIX  must contain same no of element in each column
            let x= d.len();
            let y=d[0].len();
            for i in d.iter(){
                if i.len()!=y{
                    panic!("PROVIDED SHAPE  OF MATRIX IS INCORRECT");
                }
            }
            Matris { data: d, shape: (x,y) }
        }
        pub fn gen_numericals(value:T,shape:(usize,usize))->Matris<T>{
            if shape.0<1 || shape.1<1{panic!("PROVIDED SHAPE {:?} IS IN CORRECT </ SHAPE MUST BE GREATER_EQ THEN (1,1)/>",shape)}
            Matris{
                data:vec![vec![value;shape.1];shape.0],
                shape:shape
            }
        }
        pub fn squeeze(&self)->Vec<T>{
            let mut v=Vec::with_capacity(self.shape.0*self.shape.1+2);
            for i in self.data.clone().into_iter(){
                v.extend(i)
            }
            v
        }
        pub fn reshape(&self,shape:(usize,usize))->Matris<T>{
            if self.shape.0*self.shape.1!=shape.0*shape.1{
                panic!("ENTERED SHAPE IS INVALID")
            }
            let mut v=Vec::with_capacity(shape.0*shape.1+2);
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
        pub fn resize(&mut self,shape:(usize,usize)){
            self.data=self.reshape(shape).data;
        }
        pub fn diagonal(&self)->Matris<T>{
            let (mut i,mut j)=(0 as usize,self.shape.1);
            let mut v=Vec::<Vec<T>>::new();
            for _ in 0..j{
                v.push(vec![self.data[i][j].clone()]);
            }
            let length=v.len();
            Matris { data: v, shape: (length,1 ) }
        }
        pub fn dot(&self,x:Matris<T>)->Matris<T>{
            if self.shape.0!=x.shape.1 || self.shape.1!=x.shape.0{
                panic!("shapes {:?} and {:?} not aligned: {} (dim 1) != {} (dim 0)"
                ,self.shape,x.shape,self.shape.0,x.shape.1)
            }
            let mut data=Vec::new();

            for i in self.data.iter(){
                let mut v=Vec::<T>::new();
                for j in 0..self.shape.0{
                    let mut a=vec![];
                    for k in 0..self.shape.1{
                        a.push(x.data[k][j].clone()*i[k].clone())
                    }
                    v.push(a.into_iter().sum())
                }
                data.push(v)
            }
            Matris { data: data, shape: (self.shape.0,self.shape.0) }
        }
        pub fn min(&self)->T{
            // self.squeeze().into_iter().min()
            let d=self.squeeze();
            let mut m=d[0].clone();
            for i in d.into_iter(){
                if i<m{m=i}
            }
            m
        }
        pub fn max(&self)->T{
            let d=self.squeeze();
            let mut m=d[0].clone();
            for i in d.into_iter(){
                if i>m{m=i}
            }
            m
        }
        // pub fn mean(&self)->T{
        //     let d=self.squeeze();
        //     let l=d.len() as T;
        //     d.into_iter().sum::<T>()/l
        // }
}