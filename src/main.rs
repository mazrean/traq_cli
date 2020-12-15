use openapi::apis;
use openapi::models;
use std::env::args;

#[tokio::main]
async fn main() {
    let mut default_config = apis::configuration::Configuration::default();
    default_config.base_path = String::from("https://q.trap.jp/api/v3");
    let config = default_config;

    let name = args().nth(1);
    if name == None {
        panic!("no name");
    }

    print!("password:");
    let mut input = String::new();
    let result = std::io::stdin().read_line(&mut input);
    if let Err(e) = result {
        panic!(e);
    }
    let pass = input.trim().to_string();

    let req = Some(models::PostLoginRequest::new(name.unwrap(), pass));

    let res = apis::authentication_api::login(&config, None, req).await;
    match res {
        Ok(_) => println!("login succeeded!"),
        Err(err) => {
            if let apis::Error::ResponseError(e) = err {
                if e.status.as_u16() == 401 {
                    println!("unauthorized");
                } else {
                    println!("{}:{}", e.status.as_u16(), e.content);
                }
            } else {
                panic!(err);
            }
        }
    }
}
