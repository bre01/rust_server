use std::fs;

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, StatusCode,Method,Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec,FramedRead};

static INDEX: &str = "examples/send_file_index.html";
static NOTFOUND: &[u8] = b"Not Found";
/* 
async fn handle_request(_req:Request<Body>)->Result<Response<Body>,hyper::Error >{

}
*/
async fn hello_world(_req:Request<Body> )->Result<Response<Body>,Infallible>
{
   Ok(Response::new("hello world".into()))
}

#[tokio::main]
async fn main(){
    let addr=SocketAddr::from(([127,0,0,1],3000));
    let make_svc=make_service_fn(|_conn| async
    {
        Ok::<_,Infallible>(service_fn(response_examples))
    });
    let server=Server::bind(&addr).serve(make_svc);
    
    if let Err(e)=server.await{
        eprintln!("server error: {}",e)
    }
}



async fn response_examples(req: Request<Body>) -> Result<Response<Body>,Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => simple_file_send(INDEX).await,
        (&Method::GET, "/no_file.html") => {
            // Test what happens when file cannot be be found
            simple_file_send("this_file_should_not_exist.html").await
        }
        _ => Ok(not_found()),
    }
}

/// HTTP status code 404
fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOTFOUND.into())
        .unwrap()
}

async fn simple_file_send(filename: &str) -> Result<Response<Body>,Infallible> {
    // Serve a file by asynchronously reading it by chunks using tokio-util crate.

    if let Ok(file) = File::open(filename).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(Response::new(body));
    }

    Ok(not_found())
}