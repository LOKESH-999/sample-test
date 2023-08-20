mod matrix;
fn f(x:String)->Option<i32>{
    x.trim().parse::<i32>().ok()
}
enum Type<T>{
    d(T),
}

#[derive(Debug)]
enum Fl{
    F32(f32),
    F64(f64),
    I32(i32)
}
// macro_rules! qw {
//     // (Fl::F32($s:expr))=>{$s};
//     // (Fl::F64($s:expr))=>{$s};
//     // (Fl::I32($S:expr))=>{$s};
//     ($($s:expr)=>{
       
//        })
// }
trait ll {
    fn unwrap(self);
}
fn main() {
    println!("Hello, world!");
    println!("{:?}",f("123".to_string()));
    println!("{:?}",f("12qw".to_string()));
    println!("{:?}",(0 as f64));
    println!("{:?}",matrix::Matris::gen_numericals(0 as f64, (10,3)));
    let mut  x=vec![1,23,4];
    let mut c=x.clone().into_iter().map(Fl::I32).collect::<Vec<Fl>>();
    println!("{:?}",c);
    println!("{:?}",x);
    let x=Fl::F32(5.6 as f32);
    // qw!(x);
    println!("{:?}",x);
    // c.into_iter().map(|x|{
    //     qw!(x);
    // })
}
