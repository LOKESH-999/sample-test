#[derive(Debug)]
pub struct Fraction{
    data:f64,
    int_val:i64,
    fraction_val:Option<(i64,i64)>,
    formated_val:Option<String>,
}
pub fn fraction_converter(data:f64)->Fraction{
    let y1=data.to_string().split('.').into_iter().map(|x|{x.trim().parse::<i64>().unwrap()}).collect::<Vec<i64>>();
    if y1.len()<2{
        return Fraction{data:data,
            int_val:y1[0],
            fraction_val:None,
            formated_val:None
        };
    }
    let d2=i64::pow(10,y1[1].to_string().len()as u32);
    fn f(x:i64,y:i64)->(i64,i64){
        if x==0{
            return (0,y);}
        if x%5==0 && y%5==0{
            return f(x/5,y/5);}
        if x%2==0 &&y%2==0{
            return f(x/2,y/2);}
        (x,y)}
    let d=f(y1[1],d2);
    let q=format!("{}+({}/{})",y1[0],d.0,d.1);
    println!{"{q}"};
    Fraction{
        data:data,
        int_val:y1[0],
        fraction_val:Some(d),
        formated_val:Some(q)}
}