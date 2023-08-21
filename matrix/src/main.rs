mod matrix;

fn main() {
    println!("Hello, world!");
    let mut  x=matrix::Matris::<i32>::ones((5,4));
    println!("{:?}",x.reshape( (4,5),false ));
    println!("{:?}",x);
    println!("{:?}",x.data());
    println!("{:?}",x.squeeze())
}
