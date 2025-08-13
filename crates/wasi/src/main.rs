use std::env;
use std::thread::sleep;
use std::time::{Duration, Instant};
use waki::Client;

fn main() {
	 	println!("WEB REQUEST");
    println!("---------------------------------------------------------------");
		// get with query
    let resp = Client::new()
        .get("https://httpbin.org/get?a=b")
        .headers([("Content-Type", "application/json"), ("Accept", "*/*")])
        .send()
        .unwrap();
    println!(
        "GET https://httpbin.org/get, status code: {}, body:\n{}",
        resp.status_code(),
        String::from_utf8(resp.body().unwrap()).unwrap()
    );
    println!("---------------------------------------------------------------");

		println!("Arguments:");
    println!("---------------------------------------------------------------");
		env::args().for_each(|arg| {
			println!("- {}", arg);
		});
    println!("---------------------------------------------------------------");

    println!("STDIN echo:");
    println!("---------------------------------------------------------------");
    let lines = std::io::stdin().lines();
    for line in lines {
        println!(">: {}", line.unwrap());
    }
    println!("---------------------------------------------------------------");


		println!("Environment Variables:");
    println!("---------------------------------------------------------------");
		env::vars().for_each(|(key, value)| {
			println!("- {}={}", key, value);
		});
    println!("---------------------------------------------------------------");

		println!("Timing test:");
    println!("---------------------------------------------------------------");
    let start = Instant::now();
    println!("Napped start {:?}", start);
    sleep(Duration::from_millis(100));
    println!("Napped for {:?}", Instant::now().duration_since(start));
    println!("---------------------------------------------------------------");
    eprintln!("This is in stderr!");
}
