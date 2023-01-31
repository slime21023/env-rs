use dotenv::dotenv;
use std::env;

struct Env {
    key: String,
    value: String,
}

fn args_handler(item: &String) -> Env {
    let v: Vec<&str> = item.split("=").collect();
    match v.len() {
        1 => Env {
            key: String::from(v[0]),
            value: String::from("")
        },
        _ => Env {
            key: String::from(v[0]),
            value: String::from(v[1])
        }
    }
}

fn main() {
    dotenv().ok();
    
    let args: Vec<String> = env::args().collect();
    let args_slice = &args[1..];

    let envs: Vec<Env> = args_slice.iter()
        .map(args_handler).collect();

    for env in envs {
        env::set_var(env.key, env.value);
    }
}
