


pub use super::*;


#[post("/request-new")]
pub(self) async fn request_new_event(
    req: HttpRequest,
    event_info: web::Query<models::events::EventInfoQuery>,
    app_state: web::Data<AppState>,
) -> RustackiHttpResponse{

    // extracting neccessary data storages and actors
    let storage = app_state.as_ref().app_storage.as_ref();
    let actors = app_state.actors.as_ref().unwrap();
    let pg_pool = storage.as_ref().unwrap().get_seaorm_pool().await.unwrap();
    let redis_client = storage.as_ref().unwrap().get_redis().await.unwrap();
    let lapin_pool = storage.as_ref().unwrap().get_lapin_pool().await.unwrap();
    let mut redis_conn = redis_client.get_connection().unwrap();
    let mut rmq_pool = lapin_pool.get().await.unwrap();
    let rustacki_server_actor = actors.clone().ws_actors.hoop_server_actor;
    let redis_actix_actor = storage.as_ref().unwrap().get_redis_actix_actor().await.unwrap();
    let redlock_manager = storage.as_ref().unwrap().get_locker_manager().unwrap();
    let ramdb = app_state.as_ref().ramdb.clone(); // NOTE: lock on the ramdb in a separate thread using tokio::spawn
    let redis_pool = storage.as_ref().unwrap().get_redis_pool().await.unwrap();
    let redis_conn = redis_pool.get().await;

    // use payload: Multipart to store event json data along with its picture
    // redis unique request identifier key with exp time like hash(uuid + user_id + timestamp)
    // error handling: write to file then return RustackiHttpResponse in every method, api and functions

    // the sec.rs actor checks the finish time of all events constantly 
    // to update the event status
    // ...


    // request object has lots of external methods installed on it using plugins
    // crate, we're accessing the get_user() method from the passport plugin
    match req.get_user().await{
        Ok(user_data) => {

            // event info validation
            // ...
            let event_info = event_info.0;
            let event_id = event_info.eid.as_str();

            match event_info.etype{
                Type::T1 => {

                    todo!()

                },
                Type::T2 => {

                    todo!()

                },
                Type::T3 => {

                    todo!()

                },
                _ => {

                    resp!{
                        &[u8],
                        &[],
                        None,
                        consts::INVALID_EVENT_TYPE,
                        StatusCode::NOT_ACCEPTABLE,
                        None::<Cookie<'_>>,
                    }

                }
            }

        },
        Err(err_resp) => err_resp // http response with custom error handler
    }

}