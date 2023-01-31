use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_slice = &args[1..];

    let keys =  args_slice.iter()
        .filter(|item| !item.starts_with("-"));

    for key in keys {
        match env::var(key) {
            Ok(val) => println!("{} = {}", key, val),
            Err(e) => println!("couldn't interpret {key}: {e}"),
        };
    }
}