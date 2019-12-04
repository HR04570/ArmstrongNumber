use std::io;
fn main() {
  let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
let mut _n: i32 = input.trim().parse().unwrap();
//println!("{}",n+n);
let mut _sum=0;
let mut _h=_n;
while _n!=0{
let _r=_n % 10;
_n=_n/10;
//let base: i32 = 2; // an explicit type is required
//assert_eq!(_r.pow(10), 1024);
let mut p=_h.to_string();
p=p.len().to_string();

//p.parse::<i32>().unwrap();
_sum=_sum+i32::pow(_r,p.parse::<u32>().unwrap());
//println!("{}",_r);
//n=n / 10; // an explicit type is required


}
if(_sum==_h){
println!("{} is a armstrong number",_h);
}
else
{
println!("{} is not a armstrong number",_h);

}
}