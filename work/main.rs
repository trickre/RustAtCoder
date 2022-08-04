use std::io;
fn read_line() -> String{
	let mut s = String::new();
	io::stdin().read_line(&mut s).unwrap();
	return s;
}
fn get_nums_from_line() -> Vec<i32>{
	let mut ret:Vec<i32> = Vec::new();
	let mut input_buf=String::new();
	io::stdin().read_line(&mut input_buf).unwrap();
	input_buf = input_buf.replace("\n","");
	let iv: Vec<&str> = input_buf.split(" ").collect();
	for s in iv{
		ret.push(s.parse::<i32>().unwrap());
	}
	println!("{:?}",ret);	
	//ret.push(iv[0].parse(i32).unwrap());
	//ret.push(i32(iv[1]));
	return ret;
}
fn main(){
	let line = read_line();
	print!("{}",line);
	print!("{}",read_line());
	let v = get_nums_from_line();
	print!("{} {}",v[1],v[0]);
}

