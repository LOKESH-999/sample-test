
#[derive(Debug)]
pub struct Matris{
    data:Vec<Vec<f64>>,
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
fn check_size<T>(d:&Vec<Vec<T>>)->(usize,usize){
    let x= d.len();
    let y=d[0].len();
    for i in d.iter(){
        if i.len()!=y{
            panic!("PROVIDED SHAPE  OF MATRIX IS INCORRECT");
        }
    }
    (x,y)
}

impl Matris {

        pub fn data(&self)->Vec<Vec<f64>>{
            self.data.clone()
        }
        pub fn shape(&self)->(usize,usize){
            self.shape.clone()
        }

        pub fn zeros(shape:(usize,usize))->Matris{
            Matris::gen_numericals(0.0, shape)
        }
        pub fn ones(shape:(usize,usize))->Matris{
            Matris::gen_numericals(1.0, shape)
        }
        
        pub fn gen_numericals(value:f64,shape:(usize,usize))->Matris{
            if shape.0<1 || shape.1<1{panic!("PROVIDED SHAPE {:?} IS IN CORRECT </ SHAPE MUST BE GREATER_EQ THEN (1,1)/>",shape)}
            Matris{
                data:vec![vec![value;shape.1];shape.0],
                shape:shape
            }
        }
        pub fn squeeze(&self)->Vec<f64>{
            let mut v=Vec::with_capacity(self.shape.0*self.shape.1+2);
            for i in self.data.clone().into_iter(){
                v.extend(i)
            }
            v
        }
        pub fn reshape(&self,shape:(usize,usize))->Matris{
            if self.shape.0*self.shape.1!=shape.0*shape.1{
                panic!("ENTERED SHAPE IS INVALID")
            }
            let mut v=Vec::with_capacity(shape.0*shape.1+2);
            let mut v1=Vec::<f64>::with_capacity(shape.0+2);
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
        pub fn diagonal(&self)->Matris{
            let (mut i,mut j)=(0 as usize,self.shape.1);
            let mut v=Vec::<Vec<f64>>::new();
            for _ in 0..j{
                v.push(vec![self.data[i][j].clone()]);
            }
            let length=v.len();
            Matris { data: v, shape: (length,1 ) }
        }
        pub fn dot(&self,x:Matris)->Matris{
            if self.shape.0!=x.shape.1 || self.shape.1!=x.shape.0{
                panic!("shapes {:?} and {:?} not aligned: {} (dim 1) != {} (dim 0)"
                ,self.shape,x.shape,self.shape.0,x.shape.1)
            }
            let mut data=Vec::new();

            for i in self.data.iter(){
                let mut v=Vec::<f64>::new();
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
        pub fn min(&self)->f64{
            // self.squeeze().into_iter().min()
            let d=self.squeeze();
            let mut m=d[0].clone();
            for i in d.into_iter(){
                if i<m{m=i}
            }
            m
        }
        pub fn max(&self)->f64{
            let d=self.squeeze();
            let mut m=d[0].clone();
            for i in d.into_iter(){
                if i>m{m=i}
            }
            m
        }
        pub fn mean(&self)->f64{
            let d=self.squeeze();
            let l=d.len() as f64;
            d.into_iter().sum::<f64>()/l
        }
}

pub trait  transform {
    fn from_f64(d:Vec<Vec<f64>>)->Matris;
    fn from_f32(d:Vec<Vec<f32>>)->Matris;
    fn from_u8(d:Vec<Vec<u8>>)->Matris;
    fn from_i32(d:Vec<Vec<i32>>)->Matris;
}

impl transform for Matris{
    fn from_f32(d:Vec<Vec<f32>>)->Matris {
        let (x,y)=check_size(&d);
        let mut d1=Vec::<Vec<f64>>::with_capacity(x*y);
        for i in d.into_iter(){
            d1.push(
                i.into_iter().map(|x|{x as f64}).collect::<Vec<f64>>()
            )
        }
        Matris { data: d1, shape: (x,y) }
    }

    fn from_f64(d:Vec<Vec<f64>>)->Matris{   // MATRIX  must contain same no of element in each column
        let (x,y)=check_size(&d);
        Matris { data: d, shape: (x,y) }
    }

    fn from_i32(d:Vec<Vec<i32>>)->Matris {
        let (x,y)=check_size(&d);
        let mut d1=Vec::<Vec<f64>>::with_capacity(x*y);
        for i in d.into_iter(){
            d1.push(
                i.into_iter().map(|x|{x as f64}).collect::<Vec<f64>>()
            )
        }
        Matris { data: d1, shape: (x,y) }
    }
    fn from_u8(d:Vec<Vec<u8>>)->Matris {
        let (x,y)=check_size(&d);
        let mut d1=Vec::<Vec<f64>>::with_capacity(x*y);
        for i in d.into_iter(){
            d1.push(
                i.into_iter().map(|x|{x as f64}).collect::<Vec<f64>>()
            )
        }
        Matris { data: d1, shape: (x,y) }
    }
}