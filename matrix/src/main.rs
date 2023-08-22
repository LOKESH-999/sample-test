mod matrix;

fn main() {
    // println!("Hello, world!");
    // let mut  x=matrix::ones((5,4));
    // println!("{:?}",x.reshape( (4,5)));
    // println!("{:?}",x);
    // println!("{:?}",x.data());
    // println!("{:?}",x.squeeze());
    // x.resize((4,5));
    // println!("{:?}",x);
    // let mut  x2=matrix::ones((5,4));
    // println!("{:?}",x.diagonal());
    // let x1=vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
    // println!("{:?}",x1);
    // println!("{:?}",x1);
    // println!("{:?}",x.dot(x2.reshape((4,5))));
    let q=matrix::Matris::<f64>::gen_numericals(1.1, (3,5));
    let l=vec![vec![1,2,3],vec![3,4,5]];
    let l1=vec![vec![1,2],vec![3,4],vec![5,6]];
    let a=matrix::Matris::from_vec(l);
    let a2=matrix::Matris::from_vec(l1);
    println!("{:?}",a2.dot(a));
}
