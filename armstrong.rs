use std::io;
fn main() {
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
let mut n: i32 = input.trim().parse().unwrap();
let mut sum=0;
let mut copy_number=n;
while n!=0{
	let rem=n % 10;
	n=n/10;
	let mut power=copy_number.to_string();
	power=power.len().to_string();
	sum=sum+i32::pow(rem,power.parse::<u32>().unwrap());
	}
if(sum==copy_number){
		println!("{} is a armstrong number",copy_number);
	}
else{
		println!("{} is not a armstrong number",copy_number);
	}
}