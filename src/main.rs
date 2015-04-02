use std::process::{Command, Stdio};
use std::io::{Write, Read};
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::string::String;


const BUFSIZE: usize = 16;



fn main() {


    let (sender, receiver) = channel::<String>();

    thread::spawn(move || {
        let echo = Command::new("cat")
            .arg("sample_cmd")
            .stdin(Stdio::capture())
            .stdout(Stdio::capture())
            .spawn()
            .unwrap();
        let mut echo_input = echo.stdin.unwrap();
        let mut echo_output = echo.stdout.unwrap();

        //echo_input.write(b"ls");
        //echo_input.write(b"exit");

        loop {
            let mut buf:[u8; BUFSIZE] = [0; BUFSIZE];
            let result = echo_output.read(&mut buf).unwrap();

            let msg = String::from_utf8_lossy(&buf).into_owned();


            if result != 0 {
                sender.send(msg);
            } else {
                sender.send(msg);
                break;
            }
        }
    });

    thread::scoped(move||{
        let gash = Command::new("/Users/hsiao/Dropbox/Courses/cosi-146a/cs146-pa2-shell/target/pa2")
            .stdin(Stdio::capture())
            .spawn()
            .unwrap();

        let mut gash_in = gash.stdin.unwrap();
        loop {
            match receiver.recv() {
                Ok(s) => {
                    println!("got message: {}", s);
                    gash_in.write(s.to_string().into_bytes().as_slice());
                }
                Err(_) => {
                    break;
                }
            }
        }
    });

}
