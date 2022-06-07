use std::net::{IpAddr, SocketAddr};
use rocket::outcome::Outcome::{Forward, Success};
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

pub struct MyRemoteAddr {
    pub addr: IpAddr
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MyRemoteAddr {
    type Error = !;
    
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.client_ip() {
            Some(addr) => Success(MyRemoteAddr { addr }),
            None => Forward(())
        }
    }
}
