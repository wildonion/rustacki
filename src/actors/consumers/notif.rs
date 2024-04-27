


use actix::prelude::*;
use lapin::protocol::exchange;
use std::sync::Arc;
use actix::{Actor, AsyncContext, Context};
use async_std::stream::StreamExt;
use lapin::options::{BasicAckOptions, BasicConsumeOptions, QueueBindOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::BasicProperties;
use crate::apis::http::v1::events::create;
use crate::plugins::notif::NotifExt;
use crate::s3::Storage;
use crate::consts::{self, PING_INTERVAL};
use serde::{Serialize, Deserialize};


#[derive(Message, Clone, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct ConsumeNotif{
    pub queue: String,
    pub exchange_name: String,
    pub routing_key: String,
    pub tag: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ReceiverNotif{
    receiver_info: ReceiverInfo,
    notifs: Vec<NotifData>
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ReceiverInfo{
    pub id: i32, // a unique identity
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ActionType{ // all the action type that causes the notif to get fired
    EventDetails,
    ProductPurchased,
    // probably other system notifs
    // to be sent through SSE 
    // ...
}

#[derive(Serialize, Deserialize, Clone)]
pub enum EventDetails{
    SuggestEvent,
    CreateEventDetails,
    Join,
    Left,
    Lock,
    Disable,
    Expire,
    Finish,
    Start,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum CreateEventDetails{
    T1,
    T2,
    T3,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NotifData{
    pub action_data: Option<serde_json::Value>, // any data
    pub actioner_info: Option<String>, // json stringified identifer
    pub action_type: ActionType, // type event
    pub fired_at: Option<i64>, 
    pub is_seen: bool,
}

#[derive(Clone)]
pub struct NotifConsumerActor{
    pub app_storage: std::option::Option<Arc<Storage>>,
}

impl Actor for NotifConsumerActor{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {

        log::info!("🎬 NotifConsumerActor has started, let's consume baby!");

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

impl NotifConsumerActor{

    pub fn new(app_storage: std::option::Option<Arc<Storage>>) -> Self{
        Self { app_storage }
    }

    pub async fn consume(&self, consumer_tag: &str, queue: &str, routing_key: &str, exchange: &str){

        let storage = self.app_storage.clone();
        let rmq_pool = storage.unwrap().get_lapin_pool().await.unwrap();
        match rmq_pool.get().await{
            Ok(conn) => {

                let create_channel = conn.create_channel().await;
                match create_channel{
                    Ok(chan) => {

                        // -ˋˏ✄┈┈┈┈ making a queue inside the broker per each consumer, 
                        let create_queue = chan
                            .queue_declare(
                                &queue,
                                QueueDeclareOptions::default(),
                                FieldTable::default(),
                            )
                            .await;

                        let Ok(q) = create_queue else{
                            let e = create_queue.unwrap_err();
                            use crate::error::{ErrorKind, RustackiErrorResponse};
                            let error_content = &e.to_string();
                            let error_content = error_content.as_bytes().to_vec();
                            let mut error_instance = RustackiErrorResponse::new(
                                *consts::STORAGE_IO_ERROR_CODE, // error code
                                error_content, // error content
                                ErrorKind::Storage(crate::error::StorageError::Rmq(e)), // error kind
                                "NotifConsumerActor.queue_declare" // method
                            );
                            error_instance.wirte_async().await;
                            return;
                        };

                        // binding the queue to the exchange routing key
                        /* -ˋˏ✄┈┈┈┈ 
                            if the exchange is not direct or is fanout or topic we should bind the 
                            queue to the exchange to consume the messages from the queue. binding 
                            the queue to the passed in exchange, if the exchange is direct every 
                            queue that is created is automatically bounded to it with a routing key 
                            which is the same as the queue name, the direct exchange is "" and 
                            rmq doesn't allow to bind any queue to that manually
                        */
                        if exchange != ""{
                            match chan
                                .queue_bind(q.name().as_str(), &exchange, &routing_key, 
                                    QueueBindOptions::default(), FieldTable::default()
                                )
                                .await
                                {
                                    Ok(_) => {},
                                    Err(e) => {
                                        use crate::error::{ErrorKind, RustackiErrorResponse};
                                        let error_content = &e.to_string();
                                        let error_content = error_content.as_bytes().to_vec();
                                        let mut error_instance = RustackiErrorResponse::new(
                                            *consts::STORAGE_IO_ERROR_CODE, // error code
                                            error_content, // error content
                                            ErrorKind::Storage(crate::error::StorageError::Rmq(e)), // error kind
                                            "NotifConsumerActor.queue_bind" // method
                                        );
                                        error_instance.wirte_async().await;
                                        return;
                                    }
                                }
                        }

                        // -ˋˏ✄┈┈┈┈ consuming from the queue owned by this consumer
                        match chan
                            .basic_consume(
                                queue,
                                consumer_tag, 
                                BasicConsumeOptions::default(), 
                                FieldTable::default()
                            )
                            .await
                        {
                            Ok(mut consumer) => {

                                // stream over consumer to receive data from the queue
                                while let Some(delivery) = consumer.next().await{
                                    match delivery{
                                        Ok(delv) => {

                                            // if the consumer receives the data
                                            match delv.ack(BasicAckOptions::default()).await{
                                                Ok(ok) => {

                                                    let buffer = delv.data;
                                                    let data = std::str::from_utf8(&buffer).unwrap();

                                                    log::info!("Data => {}", data);

                                                    /* 
                                                                        further operations 

                                                        step0) decode data into ProduceNotif structure
                                                        step1) get ReceiverNotif instance from redis cache 
                                                               and update notifs field by pushing the new notif
                                                               into its notifs vector then update the instance 
                                                               inside redis, if the ReceiverNotif instance is 
                                                               not in redis cache build a new instance then store
                                                               in redis for s
                                                        step2) get cached data with exp key (key: notif_id | value: Vec<NotifData> => use a redis exp key) from redis using sse
                                                        step3) store data in timescaledb with seaorm schema
                                                        seen and old notifs will be pruned per month
                                                        use mutator actors to write to db
                                                    */
                                                    // ...
                                                    
                                                },
                                                Err(e) => {
                                                    use crate::error::{ErrorKind, RustackiErrorResponse};
                                                    let error_content = &e.to_string();
                                                    let error_content = error_content.as_bytes().to_vec();
                                                    let mut error_instance = RustackiErrorResponse::new(
                                                        *consts::STORAGE_IO_ERROR_CODE, // error code
                                                        error_content, // error content
                                                        ErrorKind::Storage(crate::error::StorageError::Rmq(e)), // error kind
                                                        "NotifConsumerActor.consume_ack" // method
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
                                                "NotifConsumerActor.consume_getting_delivery" // method
                                            );
                                            error_instance.wirte_async().await;
                                            return; 
                                        }
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
                                    "NotifConsumerActor.consume_basic_consume" // method
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
                            "NotifConsumerActor.consume_create_channel" // method
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
                    "NotifConsumerActor.consume_pool" // method
                );
                error_instance.wirte_async().await;
                return;
            }
        };

    }

}

impl Handler<ConsumeNotif> for NotifConsumerActor{
    
    type Result = ();
    fn handle(&mut self, msg: ConsumeNotif, ctx: &mut Self::Context) -> Self::Result {

        // unpacking the consume data
        let ConsumeNotif { 
                queue, 
                tag,
                exchange_name,
                routing_key,
            } = msg.clone(); // the unpacking pattern is always matched so if let ... is useless
        
        let this = self.clone();
        tokio::spawn(async move{
            this.consume(&tag, &queue, &routing_key, &exchange_name).await;
        });
        
        return;
    }

}

// the ReceiverNotif struct is defined here, it's not outside of this crate, however the NotifExt trait
// is inside plugins trait, one of the trait or the struct can be outside of the current crate where
// the implementation is being occurred not both of them.
impl NotifExt<NotifData> for ReceiverNotif{
    
    type This = ReceiverNotif;

    fn extend_notifs(&mut self) -> Vec<NotifData> {
        todo!()
    }
    
    async fn get_notifs(&self) -> Vec<NotifData> {
        todo!()
    }

    fn set_notifs(&mut self) -> Vec<NotifData> {
        todo!()
    }

}