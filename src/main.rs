use std::env;

// source: https://www.youtube.com/watch?v=I7TFYa1v9xI
// this algorithm can probably implemented in binary, and might work better
// even though the algorithm isn't very well designed for computers, it is still fun to ask people to implement this, to see how they program

// the real algorithm is in the main function. this is just a helper function
// not really a root, it finds the largest power without exceeding in_val
fn find_root(in_val : u128) -> u128 {
	let mut v = in_val;
	while v * v > in_val {
		v -= 1;
	}

	return v;
}





fn main() {
	// get args
	let args: Vec<String> = env::args().collect();
	if args.len() < 2{
		print!("run wsqrt <num>\nwhere <num> is the integer you want a square root of");
		return;
	}


	// (x^2) % 10 = 0
	// x % 10 == 0
	
	// (x^2) % 10 = 2
	// x % 10 == 1 || x % 10 == 9

	// this lookup table is the inverse of the table used in the video
	// -1 = no value
	let square_lookup_table = [[0,0], [1,9], [-1,-1], [-1,-1], [2,8], [5, 5], [4,6], [-1, -1], [-1, -1], [3, 7]];


	// split input value (get last digit, get first part)
	let input_value = args[1].parse::<u128>().unwrap();
	let last_digit = input_value % 10;
	let first_part = input_value / 100;

	// get the possible last digit options as the video explained
	let last_digit_opts = square_lookup_table[last_digit as usize];
	// if none are found, error
	if last_digit_opts.contains(&-1){
		print!("error: square root is not an integer, can't calculate\n")
	}



	// find the closest power of the first part
	// might be possible with bisection
	let first_part_res = find_root(first_part);


	// if 2 options, calculate median square 5 and check if it's bigger or smaller
	let last_digit_res : u128;
	if last_digit_opts[0] == last_digit_opts[1] {
		last_digit_res = last_digit_opts[0] as u128;
	}
	
	// find last number with test, as seen in the video
	// i.e. compare 85^2 and see if it's larger than input
	// do this to check if 81^2 is the answer or 89^2 

	else {
		let test_result = first_part_res * 10 + 5;
		
		if test_result * test_result > input_value {
			last_digit_res = last_digit_opts[0] as u128;
		}
		else {last_digit_res = last_digit_opts[1] as u128;
		}
	}


	// combine
	let result = first_part_res * 10 + last_digit_res;

	println!("{}", result);
	

}
