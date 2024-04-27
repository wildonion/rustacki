




pub mod event;


#[macro_export]
macro_rules! bootstrap_grpc {
    (
        $app_state:expr
    ) => {
        {
            tokio::spawn(async move{
                let addr = format!("{}:{}", 
                        $app_state.config.as_ref().unwrap().vars.HOST, 
                        $app_state.config.as_ref().unwrap().vars.GRPC_PORT.parse::<u16>().unwrap()
                    ).parse::<SocketAddr>().unwrap();
                    server::streamer::grpc::event::server::EventServer::start(addr).await;
            });

            loop{}
        }        
    };
}