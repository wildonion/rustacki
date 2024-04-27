
use std::collections::HashMap;
use actix::{Actor, Addr};
use crate::config::{Env as ConfigEnv, Context};
use crate::config::EnvExt;
use crate::s3::Storage;
use crate::actors::consumers::notif::NotifConsumerActor;
use crate::actors::producers::notif::NotifProducerActor;
use crate::actors::cqrs::mutators::event::EventMutatorActor;
use crate::actors::cqrs::accessors::event::EventAccessorActor;
use crate::actors::ws::servers::hoop::HoopServer;
use crate::actors::sse::Broadcaster;
use serde::{Serialize, Deserialize};
use crate::types::*;
use crate::consts::*;
use crate::storage;


#[derive(Clone)]
pub struct WsActors{
    pub hoop_server_actor: Addr<HoopServer>,
}

#[derive(Clone)]
pub struct ConsumerActors{
    pub notif_actor: Addr<NotifConsumerActor>,
}

#[derive(Clone)]
pub struct ProducerActors{
    pub notif_actor: Addr<NotifProducerActor>,
}

#[derive(Clone)]
pub struct MutatorActors{
    pub event_mutator_actor: Addr<EventMutatorActor>,
}

#[derive(Clone)]
pub struct AccessorActors{
    pub event_accessor_actor: Addr<EventAccessorActor>,
}

#[derive(Clone)]
pub struct CqrsActors{
    pub mutators: MutatorActors,
    pub accessors: AccessorActors
}

#[derive(Clone)]
pub struct ActorInstaces{
    pub consumer_actors: ConsumerActors,
    pub producer_actors: ProducerActors,
    pub ws_actors: WsActors,
    pub cqrs_actors: CqrsActors,
    pub sse_actor: Addr<Broadcaster>,
}

#[derive(Clone)]
// NO need to store publisher actors cause they have one method 
// called emit which is used to publish data into redis channel
pub struct AppState{
    pub config: Option<std::sync::Arc<Context<ConfigEnv>>>,
    pub app_storage: Option<std::sync::Arc<Storage>>,
    pub actors: Option<ActorInstaces>, // redis subscriber actors
    pub ramdb: RamDb

}

impl AppState{

    pub async fn init() -> Self{

        let env = ConfigEnv::default();
        let ctx_env = env.get_vars();
        let configs = Some(
            std::sync::Arc::new(ctx_env)
        );

        let app_storage = storage!{ // this publicly has exported inside the misc so we can access it here 
            configs.as_ref().unwrap().vars.clone().DB_NAME,
            configs.as_ref().unwrap().vars.clone().DB_ENGINE,
            configs.as_ref().unwrap().vars.POSTGRES_HOST,
            configs.as_ref().unwrap().vars.POSTGRES_PORT,
            configs.as_ref().unwrap().vars.POSTGRES_USERNAME,
            configs.as_ref().unwrap().vars.POSTGRES_PASSWORD
        }.await;
        
        // publisher/producer + subscriber/consumer actor workers
        // all of the actors must be started within the context 
        // of actix runtime or #[actix_web::main]
        let hoop_ws_server_instance = HoopServer::new(app_storage.clone()).start();            
        let sse_actor_instance = Broadcaster::new(app_storage.clone()).start();
        let notif_consumer_actor = NotifConsumerActor::new(app_storage.clone()).start();
        let notif_producer_actor = NotifProducerActor::new(app_storage.clone()).start();
        let event_mutator_actor = EventMutatorActor::new(app_storage.clone()).start();
        let event_accessor_actor = EventAccessorActor::new(app_storage.clone()).start();
        
        let actor_instances = ActorInstaces{
            consumer_actors: ConsumerActors{
                notif_actor: notif_consumer_actor.clone(),
            },
            producer_actors: ProducerActors{
                notif_actor: notif_producer_actor.clone(),
            },
            ws_actors: WsActors{
                hoop_server_actor: hoop_ws_server_instance.clone()
            },
            cqrs_actors: CqrsActors{
                mutators: MutatorActors{
                    event_mutator_actor: event_mutator_actor.clone()
                },
                accessors: AccessorActors{
                    event_accessor_actor: event_accessor_actor.clone()
                }
            },
            sse_actor: sse_actor_instance.clone()
        };
        
        Self { 
            config: configs, 
            app_storage: app_storage.clone(), 
            actors: Some(actor_instances),  
            ramdb: std::sync::Arc::new(
                tokio::sync::Mutex::new(
                    HashMap::new()
                )
            )
        }

    }

}