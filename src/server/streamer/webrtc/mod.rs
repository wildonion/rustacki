




#[macro_export]
macro_rules! bootstrap_webrtc {
    (
        $app_state:expr
    ) => {
        {
            tokio::spawn(async move{

                let addr = format!(
                    "{}:{}",
                        $app_state.config.as_ref().unwrap().vars.HOST, 
                        $app_state.config.as_ref().unwrap().vars.P2P_PORT.parse::<u16>().unwrap()
                );

            });

            loop{}
        }        
    };
}