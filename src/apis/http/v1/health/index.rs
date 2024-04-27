



// demonstrating how to use custom error struct instead of using 
// a dynamic dispatch approach with Box<dyn Error> at runtime due to
// not knowing the exact type of error, this however allows us to 
// have more control over the exact source of error and can be unwrapped 
// using ? operator note that From, Debug, Display and Error traits 
// must all be implemented for the custom struct achiving this can 
// be as easy as drinking water with thiserror.


use actix_web::{cookie::Cookie, http::StatusCode};
use futures::StreamExt;
use lockers::llm::Product;
use log::error;
use plugins::purchase::AtomicPurchase;
pub use super::*;
use models::resp;
use crate::error::*; // :: refers to all crates loaded in Cargo.toml


async fn open_file() -> Result<(), crate::error::FileEror>{

    // ? needs to know the exact source of error to build an instance of the 
    // custom error (FileEror in our case) from it so in order to use the ? 
    // operator the From<std::io::Error> trait must be implemented for our 
    // custom error handler or FileEror so Rust can create the error by calling 
    // the from() method on the FileEror and pass the opening file process error 
    // which in our case is of type std::io::Error to it to build the FileError 
    // instance, take note that From<std::io::Error> must be implemented for 
    // FileError to do so.
    let f = std::fs::File::open("openme.txt")?;
    Ok(())

}


// more tcp logics in: https://github.com/wildonion/rustacki/blob/main/src/helpers/tcpserver.rs
#[post("/test-stream")]
pub(self) async fn test_stream(
    // payload and multipart are both in form of bytes that 
    // can be collected using while let some streaming
    req: HttpRequest,
    mut stream: web::Payload,
    // json_body: web::Json<LoginInfoRequest>,
    // some_path: web::Path<(String, i32)>,
    // multipart_body: Multipart,
    app_state: web::Data<AppState>
) -> Result<actix_web::HttpResponse, crate::error::RustackiErrorResponse>{

    // streaming over the incoming binary data from client
    // later on we can map the buffer into its related strucutre
    // this is used to stream ws connection
    let mut buffer = vec![];
    while let Some(chunk) = stream.next().await{
        let bytes = chunk.unwrap();
        buffer.extend_from_slice(bytes.chunk());
    }
    
    // building the error of read/write file manually so we could return 
    // RustackiErrorResponse in respond to the client since the type of Error 
    // part of open_file() method is the custom FileError type, we can't 
    // return it in this method which it's error part is of type RustackiErrorResponse
    // we should build the RustackiErrorResponse manually from FileError type
    // ...
    // note that in the following method we've used the FileEror as the error part
    // of the result type which unwrap the error by using ? to log the exact caused 
    // of error to the console but note that can't use ? in here cause ? unwrap the
    // the error into RustackiErrorResponse not the its KindaError enum variant, we use
    // match in here to catch the error
    match open_file().await{
        Ok(_) => {},
        // build a custom http response from the FileError variant
        // ...
        Err(e) => { // as we can see the error type is a FileError which is one the variant of the ErrorKind enum
            // e.to_string() is the display message of the error, note without 
            // matching over the result and use unwrap() only the app gets crashed 
            // at runtime and logs the fulfilled buffer inside the Debug trait the 
            // fmt() method like so:
            // [FILE] - failed to read from or write to file
            // Caused by: 
            // No such file or directory (os error 2)
            // cause this api method requires an error type of RustackiErrorResponse
            let source_error = e.source().unwrap().to_string(); // get the exact source of the error caused by the file opening io process
            error!("{:?}", source_error);
            let err = crate::error::RustackiErrorResponse::from((
                source_error.as_bytes().to_vec(), 
                0, 
                crate::error::ErrorKind::File(e),
                String::from("")
            ));
            return Ok(err.error_response());
        }
    };


    // since we're handling the error using RustackiErrorResponse there is no need to match over
    // ok or the err part of the result we can directly use ? operator Rust will take care of 
    // the rest process.
    // we can use ? operator since the From<std::io::Error> trait has implemented for the RustackiErrorResponse
    // runtime ERROR: cause file doesn't exist
    let f = std::fs::File::open("openme.txt")?; // using ? convert the error into our custom http response error so we're not worry about making a custom http response containing the error 

    // extracting multipart formdata
    // let extracted_multipart = multipartreq::extract(
    //     std::sync::Arc::new(
    //         tokio::sync::Mutex::new(multipart_body)
    //     )
    // ).await.unwrap();
    // let json_value_formdata = extracted_multipart.0;
    // let files = extracted_multipart.1;

    // getting the json body
    // let json_body = json_body.to_owned();

    resp!{
        usize, // the data type
        buffer.len(), // response data
        None, // metadata
        &format!("Stream Length Fetched"), // response message
        StatusCode::OK, // status code
        None::<Cookie<'_>>, // cookie
    }

}


#[get("/check")]
pub(self) async fn check(
    req: HttpRequest,
    app_state: web::Data<AppState>
    // probably web::Path, web::Json, web::Query params
    // ...
) -> RustackiHttpResponse{

    resp!{
        &[u8],
        &[],
        None, // metadata
        &String::from("Alive"),
        StatusCode::OK,
        None::<Cookie<'_>>,
    }

}

#[post("/mint-demo")]
pub(self) async fn mint_demo(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    pinfo: web::Json<lockers::llm::Product>,
    // probably other web::Path, web::Json, web::Query params
    // ...
) -> RustackiHttpResponse{

    let product = pinfo.to_owned();

    tokio::spawn(async move{
        
        // none mutex checking: 
        // check is already minted in db or not
        // product amount validation

    });
    
    let (minting_exclusion, mut product_receiver) = product.atomic_purchase_status().await;

    match minting_exclusion{
        true => { // product is being minted and is locked
            resp!{
                &[u8],
                &[],
                None, // metadata
                &String::from("ERROR: pid is currently locked"),
                StatusCode::NOT_ACCEPTABLE,
                None::<Cookie<'_>>,
            }
        },
        false => { // product can be minted

            let mut product_info = Product::default();
            while let Some(prod_info) = product_receiver.recv().await{
                log::info!("product info received {:?}", prod_info);
                product_info = prod_info;
            }

            if !product_info.is_minted{
                resp!{
                    &[u8],
                    &[],
                    None, // metadata
                    &format!("ERROR: product {} wasn't minted", product.pid),
                    StatusCode::OK,
                    None::<Cookie<'_>>,
                }
            }

            // meanwhile in the background without blocking the current thread
            tokio::spawn(
                {
                    let prod_info = product_info.clone();
                    async move{

                        // send notifs through SSE:
                        // 0 - store product_info in db
                        // 1 - publish minted product info using publishers/notif.rs
                        
                        // 2 - receive product info inside subscribers/notif.rs actor
                        // 3 - send message to subscribers/notif.rs actor from sse actor 
                        //     with subscribe_notif() method to get the product info
                        // 4 - inside sse actor send received product info as notif to the client 
                        //     we'll notify that client with the minting status in realtime 
                        //     also we can send the info to the user's telegram throughout the bot

                    }

                }
            );

            resp!{
                &[u8],
                &[],
                None, // metadata
                &format!("SUCCESS: product {} is getting ready on our server, notify you about its status", product.pid),
                StatusCode::OK,
                None::<Cookie<'_>>,
            }
        }

    }

}

pub mod exports{
    pub use super::test_stream;
    pub use super::check;
    pub use super::mint_demo;
}