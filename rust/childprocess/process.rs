use std::process::{Command,Stdio};
use std::io::Write;
use std::io::Read;

fn main() {
	let process = Command::new("cat")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.expect("failed to spawn");

	let mut s1 = process.stdin.unwrap();
	let mut s2 = process.stdout.unwrap();

	let mut buf = [0 as u8; 1024];

	s1.write_all(b"test message").unwrap();
	match s2.read(&mut buf) {
		Ok(n) => {
            if n > 0 { 
                println!("{}", std::str::from_utf8(&buf[0..n]).unwrap())
            }
        },
		Err(_) => panic!("read error\n"),
	}
}
