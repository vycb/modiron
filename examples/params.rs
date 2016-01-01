extern crate iron;
extern crate params;

use iron::prelude::*;
use iron::status;
use params::Params;

fn handle(req: &mut Request) -> IronResult<Response> {
    println!("{:?}", req.get_ref::<Params>().unwrap());
    let pm = req.get_ref::<Params>().unwrap();

    for (key, value) in pm.iter() {
        println!("{:?}: {:?}", key, value);
    }
    Ok(Response::with(status::Ok))
}

fn main() {
    Iron::new(Chain::new(handle)).http("wram:8080").unwrap();
}
