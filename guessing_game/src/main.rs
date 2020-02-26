use std::io;
use std::cmp;
use rand::Rng;


fn main() {

    // Generate the secret number
    let secret = rand::thread_rng ().gen_range (1, 101);
    println! ("The secret number is {}", secret);   

    loop {
	// Get the number from the user
	println! ("Guess a number ->");
	// Decl and init the variable
	let mut guess = String::new ();
	// Read in (like fgets?)
	io::stdin ().read_line (&mut guess)
	    .expect ("Error: failed to read line");
	// Cast to unsigned 32 bit
	// match block for the sake of safety
	// Result type can handle errors
	let guess: u32 = match guess.trim ().parse () {
	    Ok (num) => num,
	    Err (_) => continue
	};
	   
	
	println! ("You guessed: {}", guess);

	// pattern matching? (lol)
	match guess.cmp (&secret) {
	    cmp::Ordering::Less => println! ("Too small"),
	    cmp::Ordering::Greater => println! ("Too big"),
	    cmp::Ordering::Equal => {
		println! ("nice");
		break;
	    }
	}
    }
    return;
}
