


use std::sync::Arc;
use actix::{Actor, AsyncContext, Context};
use crate::s3::Storage;
use crate::consts::PING_INTERVAL;


#[derive(Clone)]
pub struct EventAccessorActor{
    pub app_storage: std::option::Option<Arc<Storage>>,
}

impl Actor for EventAccessorActor{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {

        log::info!("🎬 EventAccessorActor has started, let's read baby!");

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

impl EventAccessorActor{

    pub fn new(app_storage: std::option::Option<Arc<Storage>>) -> Self{
        Self { app_storage }
    }
    
}