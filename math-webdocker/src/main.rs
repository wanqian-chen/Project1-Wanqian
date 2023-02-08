/* An actix Microservice that has multiple routes:
A. / that returns a hello world message
B. /hello/{name} that returns a hello message
C. /delete_zero/{v} that returns a vector without zero
D. /coin/{probability} that returns 1 or 0
*/

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
// import the hello function from the lib.rs file
use math_webdocker::{coin, delete_zero, hello};

// create a function that returns a hello world message
#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello Stranger! Please enter your name! (e.g. /hello/Wanqian)")
}

// create a hello function that returns a hello message
#[get("/hello/{name}")]
async fn hello_name(name: web::Path<String>) -> impl Responder {
    let name = hello(&name);
    HttpResponse::Ok().body(format!("Hello {}!", name))
}

// // create a function that returns a vector without zero
// #[get("/delete-zero/{v}")]
// async fn delete_zero_v(v: web::Path<String>) -> impl Responder {
//     // convert the string to a vector
//     let v: Vec<i32> = v.split("-").map(|s| s.parse().unwrap()).collect();
//     let v = delete_zero(&mut v.clone());
//     HttpResponse::Ok().body(format!("The vector without zero is {:?}", v))
// }

// create a function that returns a coin flip
#[get("/coin/{probability}")]
async fn coin_flip(probability: web::Path<f64>) -> impl Responder {
    let probability_str = probability.to_string();
    let probability: f64 = match probability_str.parse() {
        Ok(val) => val,
        Err(_) => return HttpResponse::BadRequest().body("Invalid number"),
    };

    // convert struc to float
    let get_coin: i32 = coin(probability);
    HttpResponse::Ok().body(format!("The coin flip is {}", get_coin))
}

// main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // add a print message to the console that the server is starting
    println!("Starting server at http://localhost:8080");
    // start the server
    HttpServer::new(|| App::new().service(hello_world).service(hello_name))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
