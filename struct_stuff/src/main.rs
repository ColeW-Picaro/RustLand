// Author: Cole Vohs
// Project: struct_stuff
// Description: messing with structs in rust
use std::io;

#[derive(Clone)]
struct Student {
    fname: String,
    lname: String,
    email: String,
    number: usize,
}

fn main() {
    let mut num: usize = 0;
    let mut stu_vec: Vec<Student> = std::vec::Vec::<Student>::new();
    loop {
	// Input garbage; need to flush stdout because print! sucks? lol
        let mut name = String::new();
        print!("Name: ");
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin().read_line(&mut name).expect("Error: read_line");
        println!();

        let mut email = String::new();
        print!("Email: ");
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin().read_line(&mut email).expect("Error: read_line");
        println!();

        // Build the student; use .clone to push it into the vector
        let student: Student = build_student(&name, &email, &mut num);
        stu_vec.push(student.clone());

        // We can use the student variable here because we didn't move it
        println!("added {} {} with student number {}", student.fname, student.lname, student.number);
    }
}

fn build_student(name: &str, email: &str, num: &mut usize) -> Student {
    let name = build_name(name);
    let s = Student {
        // Not sure if .to_owned () is the correct way to do this
        fname: name.0.to_owned(),
        lname: name.1.to_owned(),
        email: email.to_owned(),
        number: *num,
    };
    *num = *num + 1;
    s
}

fn build_name(name: &str) -> (&str, &str) {
    let bytes = name.as_bytes();
    // Using ranges and slices to construct a str str tuple from one str
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&name[0..i], &name[(i + 1)..name.len()])
        }
    }
    (&name[..], &name[..])
}
