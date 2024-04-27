


use crate::*;


// config is a mutable pointer to the web::ServiceConfig
// which makes to return nothing from each functions cause
// the state of the actual instance of web::ServiceConfig
// will be mutated in its scope


/*
     --------------------------------
    |     REGISTER EVENTS ROUTES
    | -------------------------------
    |
    |

*/
pub fn init(config: &mut web::ServiceConfig){

    config.service(apis::http::v1::events::create::request_new_event);

}