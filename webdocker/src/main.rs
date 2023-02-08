/* An actix Microservice that has multiple routes:
A. / that returns a hello world message
B. /fruit that returns a random fruit
C. /health that returns a 200 status code
D. /num/fruits/{num} that returns multiple random fruits
E. /version that returns the version of the service
*/

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
// import the random_fruit function from the lib.rs file
use webdocker::random_fruit;

// create a function that returns a hello world message
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

// create a function that returns a random fruit
#[get("/fruit")]
async fn fruit() -> impl Responder {
    // print the random fruit to the console
    println!("Random Fruit: {}", random_fruit());
    HttpResponse::Ok().body(random_fruit())
}

// create a function that returns a 200 status code
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

// // create a function that returns multiple random fruits
// #[get("/num/fruits/{num}")]
// async fn num_fruits(web::Path(num): web::Path<u32>) -> impl Responder {
//     // store multiple fruits in a vector
//     let all_fruits = (0..num)
//         .map(|_| random_fruit())
//         .collect::<Vec<String>>()
//         .join("");
//     let mut fruits = String::new();
//     // loop through the number of fruits
//     for _ in 0..num {
//         // add a random fruit to the fruits string
//         fruits.push_str(&all_fruits);
//     }
//     HttpResponse::Ok().body(fruits)
// }

// create a function that returns the version of the service
#[get("/version")]
async fn version() -> impl Responder {
    // print the version of the service to the console
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // add a print message to the console that the server is starting
    println!("Starting server!");
    // start the server
    HttpServer::new(|| {
        App::new()
            // add the routes to the server
            .service(hello)
            .service(fruit)
            .service(health)
            // .service(num_fruits)
            .service(version)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
