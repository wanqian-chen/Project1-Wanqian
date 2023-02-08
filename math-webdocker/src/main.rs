/* An actix Microservice that has multiple routes:
A. / that returns a hello world message
B. /hello/{name} that returns a hello message
C. /delete_zero/{v} that returns a vector without zero
D. /coin/{probability} that returns 1 or 0
*/

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
// import the hello function from the lib.rs file
use math_webdocker::{hello, coin, delete_zero, mean, median, mode, variance, std, chi_square};

// create a function that returns a hello world message
#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello Stranger! Please enter your name! (e.g. /hello/Wanqian)")
}

// create a hello function that returns a hello message
#[get("/hello/{name}")]
async fn hello_name(name: web::Path<String>) -> impl Responder {
    let name = hello(&name);
    HttpResponse::Ok().body(format!("{}!", name))
}

// create a function that returns a vector without zero
#[get("/nozero/{string}")]
async fn delete_zero_here(string: web::Path<String>) -> impl Responder {
    // convert string to vector
    let mut v: Vec<i32> = string
        .split("-")
        .map(|s| s.parse().unwrap())
        .collect();
    let new_v = delete_zero(&mut v);
    HttpResponse::Ok().body(format!("The nozero string is {:?}", new_v))
}

// create a function that returns a coin flip
#[get("/coin/{probability}")]
async fn coin_flip(probability: web::Path<String>) -> impl Responder {
    // convert string to float
    let probability: f64 = probability.parse().unwrap();
    let result = coin(probability);
    HttpResponse::Ok().body(format!("The coin is {}", result))
}

// create a function that returns a mean
#[get("/mean/{string}")]
async fn mean_here(string: web::Path<String>) -> impl Responder {
    // convert string to vector
    let v: Vec<i32> = string
        .split("-")
        .map(|s| s.parse().unwrap())
        .collect();
    let result = mean(&v);
    HttpResponse::Ok().body(format!("The mean is {}", result))
}

// create a function that returns a median
#[get("/median/{string}")]
async fn median_here(string: web::Path<String>) -> impl Responder {
    // convert string to vector
    let v: Vec<i32> = string
        .split("-")
        .map(|s| s.parse().unwrap())
        .collect();
    let result = median(&v);
    HttpResponse::Ok().body(format!("The median is {}", result))
}

// create a function that returns a mode
#[get("/mode/{string}")]
async fn mode_here(string: web::Path<String>) -> impl Responder {
    // convert string to vector
    let v: Vec<i32> = string
        .split("-")
        .map(|s| s.parse().unwrap())
        .collect();
    let result = mode(&v);
    HttpResponse::Ok().body(format!("The mode is {:?}", result))
}

// create a function that returns a variance
#[get("/variance/{string}")]
async fn variance_here(string: web::Path<String>) -> impl Responder {
    // convert string to vector
    let v: Vec<i32> = string
        .split("-")
        .map(|s| s.parse().unwrap())
        .collect();
    let result = variance(&v);
    HttpResponse::Ok().body(format!("The variance is {}", result))
}

// create a function that returns a standard deviation
#[get("/std/{string}")]
async fn std_here(string: web::Path<String>) -> impl Responder {
    // convert string to vector
    let v: Vec<i32> = string
        .split("-")
        .map(|s| s.parse().unwrap())
        .collect();
    let result = std(&v);
    HttpResponse::Ok().body(format!("The standard deviation is {}", result))
}

// create a function that returns a chi square
#[get("/chi_square/{string}")]
async fn chi_square_here(string: web::Path<String>) -> impl Responder {
    // convert string to vector
    let v: Vec<i32> = string
        .split("-")
        .map(|s| s.parse().unwrap())
        .collect();
    let result = chi_square(&v);
    HttpResponse::Ok().body(format!("The chi square is {}", result))
}

// main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // add a print message to the console that the server is starting
    println!("Starting server at http://localhost:8080");
    // start the server
    HttpServer::new(|| App::new()
        .service(hello_world)
        .service(hello_name)
        .service(delete_zero_here)
        .service(coin_flip)
        .service(mean_here)
        .service(median_here)
        .service(mode_here)
        .service(variance_here)
        .service(std_here)
        .service(chi_square_here)
    )
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
