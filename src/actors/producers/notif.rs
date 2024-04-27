


use crate::*;
use actix::prelude::*;
use actix::{AsyncContext, Context};
use actix_redis::{resp_array, Command, RespValue};
use config::EnvExt;
use consts::PING_INTERVAL;
use lapin::options::{ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions};
use lapin::protocol::exchange;
use lapin::types::FieldTable;
use lapin::BasicProperties;
use plugins::*;
use plugins::notif::NotifExt;
use std::sync::Arc;
use crate::actors::consumers::notif::NotifData;
use crate::actors::consumers::notif::ReceiverInfo;



#[derive(Message, Clone, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct ProduceNotif{
    pub notif_receiver: ReceiverInfo,
    pub notif_data: NotifData,
    pub exchange_name: String,
    pub exchange_type: String,
    pub routing_key: String,
}

#[derive(Clone)]
pub struct NotifProducerActor{
    pub app_storage: Option<Arc<s3::Storage>>,
}

impl Actor for NotifProducerActor{
    
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {

        log::info!("🎬 NotifProducerActor has started, let's produce baby!");

        ctx.run_interval(PING_INTERVAL, |actor, ctx|{
            
            let this = actor.clone();

            tokio::spawn(async move{

                // check something constantly, schedule to be executed 
                // at a certain time
                // ...
                
            });

        });

    }
}

impl NotifProducerActor{

    pub async fn produce(&self, data: &str, exchange: &str, routing_key: &str, exchange_type: &str){

        let this = self.clone();

        // these are must be converted into String first to make longer lifetime 
        // cause &str can't get moved into tokio spawn as its lifetime it's not 
        // static the tokio spawn lives longer than the &str and the &str gets 
        // dropped out of the ram once the function is finished with executing
        let exchange = exchange.to_string();
        let routing_key = routing_key.to_string();
        let exchange_type = exchange_type.to_string();
        let data = data.to_string();

        tokio::spawn(async move{

            let storage = this.clone().app_storage.clone();
            let rmq_pool = storage.unwrap().get_lapin_pool().await.unwrap();
            
            // trying to ge a connection from the pool
            match rmq_pool.get().await{
                Ok(pool) => {

                    // -ˋˏ✄┈┈┈┈ creating a channel in this thread
                    match pool.create_channel().await{
                        Ok(chan) => {

                            // -ˋˏ✄┈┈┈┈ creating exchange
                            match chan
                                .exchange_declare(&exchange, {
                                    match exchange_type.as_str(){
                                        "fanout" => lapin::ExchangeKind::Fanout,
                                        "direct" => lapin::ExchangeKind::Direct,
                                        "headers" => lapin::ExchangeKind::Headers,
                                        _ => lapin::ExchangeKind::Topic,
                                    }
                                }, 
                                    ExchangeDeclareOptions::default(), FieldTable::default()
                                )
                                .await
                                {
                                    Ok(ex) => ex,
                                    Err(e) => {
                                        use crate::error::{ErrorKind, RustackiErrorResponse};
                                        let e_string = &e.to_string();
                                        let error_content = e_string.as_bytes().to_vec();
                                        let mut error_instance = RustackiErrorResponse::new(
                                            *consts::STORAGE_IO_ERROR_CODE, // error code
                                            error_content, // error content
                                            ErrorKind::Storage(crate::error::StorageError::Rmq(e)), // error kind
                                            "NotifConsumerActor.exchange_declare" // method
                                        );
                                        error_instance.wirte_async().await;
                                        return;   
                                    }

                                };


                            // -ˋˏ✄┈┈┈┈ publishing to exchange from this channel,
                            // later consumer bind its queue to this exchange and its
                            // routing key so messages go inside its queue, later they 
                            // can be consumed from the queue by the consumer
                            use lapin::options::BasicPublishOptions;
                            let payload = data.as_bytes();
                            match chan
                                .basic_publish(
                                    &exchange, // the way that message gets routed to the queue based on a unique routing key
                                    &routing_key, // queue must be declared and existed inside the broker
                                    BasicPublishOptions::default(),
                                    payload, // this is the ProduceNotif data,
                                    BasicProperties::default(),
                                )
                                .await
                                {
                                    Ok(pc) => {
                                        let get_confirmation = pc.await;
                                        let Ok(confirmation) = get_confirmation else{
                                            use crate::error::{ErrorKind, RustackiErrorResponse};
                                            let error_content_ = get_confirmation.unwrap_err();
                                            let e_string = &error_content_.to_string();
                                            let error_content = e_string.as_bytes().to_vec();
                                            let mut error_instance = RustackiErrorResponse::new(
                                                *consts::STORAGE_IO_ERROR_CODE, // error code
                                                error_content, // error content
                                                ErrorKind::Storage(crate::error::StorageError::Rmq(error_content_)), // error kind
                                                "NotifConsumerActor.get_confirmation" // method
                                            );
                                            error_instance.wirte_async().await;
                                            return;
                                        };

                                        if confirmation.is_ack(){
                                            log::info!("publisher sent data");
                                        }

                                    },
                                    Err(e) => {
                                        use crate::error::{ErrorKind, RustackiErrorResponse};
                                        let error_content = &e.to_string();
                                        let error_content = error_content.as_bytes().to_vec();
                                        let mut error_instance = RustackiErrorResponse::new(
                                            *consts::STORAGE_IO_ERROR_CODE, // error code
                                            error_content, // error content
                                            ErrorKind::Storage(crate::error::StorageError::Rmq(e)), // error kind
                                            "NotifConsumerActor.basic_publish" // method
                                        );
                                        error_instance.wirte_async().await;
                                        return;
                                    }
                                }

                        },
                        Err(e) => {
                            use crate::error::{ErrorKind, RustackiErrorResponse};
                            let error_content = &e.to_string();
                            let error_content = error_content.as_bytes().to_vec();
                            let mut error_instance = RustackiErrorResponse::new(
                                *consts::STORAGE_IO_ERROR_CODE, // error code
                                error_content, // error content
                                ErrorKind::Storage(crate::error::StorageError::Rmq(e)), // error kind
                                "NotifConsumerActor.create_channel" // method
                            );
                            error_instance.wirte_async().await;
                            return;
                        }
                    }
                },
                Err(e) => {

                    use crate::error::{ErrorKind, RustackiErrorResponse};
                    let error_content = &e.to_string();
                    let error_content = error_content.as_bytes().to_vec();
                    let mut error_instance = RustackiErrorResponse::new(
                        *consts::STORAGE_IO_ERROR_CODE, // error code
                        error_content, // error content
                        ErrorKind::Storage(crate::error::StorageError::RmqPool(e)), // error kind
                        "NotifConsumerActor.produce_pool" // method
                    );
                    error_instance.wirte_async().await;
                    return;
                }
            };
            
        });
        
    
    }

    pub fn new(app_storage: std::option::Option<Arc<s3::Storage>>) -> Self{
        Self { app_storage }
    }

}

impl Handler<ProduceNotif> for NotifProducerActor{
    
    type Result = ();
    fn handle(&mut self, msg: ProduceNotif, ctx: &mut Self::Context) -> Self::Result {

        // unpacking the notif data
        let ProduceNotif { 
                notif_receiver, 
                notif_data, 
                exchange_name,
                exchange_type,
                routing_key
            } = msg.clone();
        
        let stringified_data = serde_json::to_string_pretty(&msg).unwrap();
        
        let this = self.clone();
        tokio::spawn(async move{
            this.produce(&stringified_data, &exchange_name, &routing_key, &exchange_type).await;
        });
        
        return;
    }

}