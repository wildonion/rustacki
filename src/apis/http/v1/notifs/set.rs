


// receive new data from the http body then store in db using mutator actors
// it then sends notif message with ProduceNotif struct to producer actor
// then producer actor send notif to the exchange inside the rmq broker
// req body: exchange_name, exchange_type, routing_key, queue
/* 
                USE THE FOLLOWING PATTERN TO SEND NOTIF 
    
    tokio::spawn(
        {
            let cloned_app_state = app_state.clone();
            async move{
            // producing nofit by sending the ProduceNotif message to
            // the producer actor,
            let produce_notif_resp 
                = cloned_app_state.clone().actors.unwrap()
                    .producer_actors.notif_actor.send(
                        ProduceNotif{ 
                                notif_receiver: ReceiverInfo{ id: 1 },
                                notif_data: NotifData{ 
                                    action_data: Some(serde_json::json!({
                                        "pid": 200
                                    })), 
                                    actioner_info: Some(String::from("wildonion is purchasing the product")), 
                                    action_type: ActionType::ProductPurchased, 
                                    fired_at: Some(chrono::Local::now().timestamp()), 
                                    is_seen: false
                                }, 
                                exchange_name: String::from("wildonionExchange"),
                                exchange_type: String::from("amq.topic"), // amq.topic for pubsub
                                routing_key: String::from("accept.only.*") // routing pattern or key
                            }
                        )
                        .await;
            }
        }
    );

                USE THE FOLLOWING PATTERN TO CONSUME NOTIF

    tokio::spawn(
        {
            let cloned_app_state = app_state.clone();
            async move{
                // consuming notif by sending the ConsumeNotif message to 
                // the consumer actor,
                let consume_notif_resp
                    = cloned_app_state.clone().actors.unwrap()
                        .consumer_actors.notif_actor.send(
                            ConsumeNotif{ 
                                    tag: String::from("cons_tag0"),
                                    /* -ˋˏ✄┈┈┈┈ 
                                        following queue gets bounded to the passed in exchange type with its 
                                        routing key, when producer wants to produce notif data it sends them 
                                        to the exchange with a known routing key, any queue that is bounded 
                                        to that exchange routing key will be filled up with messages coming 
                                        from the producer and they stay in there until a consumer read them
                                    */
                                    queue: String::from("WildonionQueue"), 
                                    exchange_name: String::from("wildonionExchange"),
                                    routing_key: String::from("accept.only.*")
                                }
                        )
                        .await;

            }
        }
    );
    
*/