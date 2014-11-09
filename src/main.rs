extern crate nickel;
extern crate http;

use std::collections::HashMap;
use std::io::net::ip::Ipv4Addr;
use nickel::{ Nickel, Request, Response, HttpRouter,StaticFilesHandler };

mod controller;


fn main () {
    let mut server = Nickel::new();
    server.utilize(StaticFilesHandler::new("templates/"));
    server.get("/", index_handler);
    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}


fn index_handler (_request: &Request, response: &mut Response) { 
    let mut data = HashMap::<&str, &str>::new();
    data.insert("name", "user128");
    controller::hello();
    response.render("templates/index.html", &data);
    //        response.send("hello world"); 
}

fn exec_handler (_request: &Request, response: &mut Response) {
    
}



