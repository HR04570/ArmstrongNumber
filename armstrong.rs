use std::io;
fn main() {
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
let mut _n: i32 = input.trim().parse().unwrap();
let mut _sum=0;
let mut _h=_n;
while _n!=0{
let _r=_n % 10;
_n=_n/10;
let mut p=_h.to_string();
p=p.len().to_string();
_sum=_sum+i32::pow(_r,p.parse::<u32>().unwrap());
}
if(_sum==_h){
println!("{} is a armstrong number",_h);
  }
else{
println!("{} is not a armstrong number",_h);
  }
}