mod matrix;

fn main() {
    println!("Hello, world!");
    let mut  x=matrix::Matris::<i32>::ones((5,4));
    println!("{:?}",x.reshape( (4,5)));
    println!("{:?}",x);
    println!("{:?}",x.data());
    println!("{:?}",x.squeeze());
    x.resize((4,5));
    println!("{:?}",x);
    println!("{:?}",x.diagonal());
    let x=vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
    println!("{:?}",x);
    println!("{:?}",x);
}
