

## ᝰ.ᐟ What am i?

you probably might be asking yourself what am i? well i'm an stateful distributed backend boilerplate with an onion design pattern upon great stacks leveraging the power of streaming technologies by utilising redis and rmq so just code me! i have a supper clean code structure and actor based components which made me a ready to up microservice.
don't afraid of changing my structure, understanding me is as quite dead simple as drinking water.

## Execution flow & system design?

> this boilerplate can be either a producer or consumer actor worker upon supported protocols (`http`, `grpc`, `tcp`, `webrtc`, `p2p`) or a single worker service.

### As a producer/publisher (register notif) actor worker service: 

send data through tcp or http request to this service (`notifs/set.rs`), server broadcasts message with rmq producer it then stores data in db for future reports (`notifs/get.rs`) and respond the caller with an ok status. producing or publishing process is done based on an event that is occurred during the lifetime of the app execution.

### As a consumer/subscriber actor worker service:

server consume or subscribe to data topics by creating channel per thread which contains message frame then binds a queue to the specified topic with the given routing key, this service can be used to send received data to frontend in realtime through ws connection or http sse, in this service we'll store in timescaledb and cache received data in redis with an expiration key, all consuming and subscribing processes.

### What about frontend service:

read directly from the rmq ws broker itself for realtime monitoring in frontend side.

### 🎬 Actor worker communication flow:

> message brokers like rmq components are actor workers wich use internal rpc to talk remotely with the broker like creating queue, exchange, channels and exchangeing messages between producers and consumers based on various exchange patterns, the broker however communicate with client over tcp and mainly contains message queue, exchange, routing and binding strcutures and strategies to make a reliable message exchanging protocol.

**channels:** create channel per thread cause they’re not safe to be shared, use channel to send message to them, they have mailbox and queues and routing strategies to execute async messages and tasks.

**threadpool:** they execute async tasks like mutex atomic syncing operations within tokio spawn or their own builtin threadpool and control the flow of the code with tokio select event loop and channels.

**communication:** local msg handlers backed by channel (mpsc) based job queues, remote msg handlers backed by rpc based job queues.

**task scheduling:** if you want to check or execute something constantly in the background inside a threadpool actor is the beast to do so. 

## Streaming stack!?

- **Rabbitmq** message broker for PubSub and ProducerConsumer patterns.
- **Postgres** with **timescaledb** extension to store seamlessly.
- **Redis** mainly for expire-caching notif data.

## Monitoring stack!?

- **dbeaver** db administration tool.
- **adminer** postgres admin panel.
- **portainer** docker container manager.
- **grafana** analytics visualization.

<p align="center">
    <img src="https://github.com/wildonion/rustacki/blob/main/infra/arch.png">
</p>

## How 2 setup, develop, and deplou?

> if you want to deploy as a publisher or producer service then get your hands dirty by developing the `apis/http`, `actors/producers`, as a subscriber or consumer however, develop the `apis/http`, `actors/consumers` folders.

### Dev env

```bash
# launch as http
cargo run --bin rustacki -- --server http # default
# launch as tcp
cargo run --bin rustacki -- --server tcp
# launch as grpc
cargo run --bin rustacki -- --server grpc
# launch as webrtc
cargo run --bin rustacki -- --server webrtc
# launch as quic
cargo run --bin rustacki -- --server quic
# launch as p2p
cargo run --bin rustacki -- --server p2p
# launch as worker
cargo run --bin rustacki -- --server worker
# or see help
cargo run --bin rustacki -- --help
```

### Prod env

> current nginx password for `logs` dir is `rustacki@1234` also make sure you've changed the `your-app.app` to your own domain in every where mostly the nginx config files and changed the `APP_NAME` in `consts` to your desired app name.

#### step1) setup vps

```bash
# -----------------------
# ---- read/write access
sudo chmod +x /root && sudo chown -R www-data:www-data /root && sudo chmod -R 777 /root
sudo gpasswd -a www-data root && sudo chmod g+x /root && sudo -u www-data stat /root
sudo chown -R root:root /root && sudo chmod -R 777 /root
sudo chown -R www-data:www-data . && sudo chmod +x /root
sudo chown -R root:root . && sudo chmod -R 777 . 
sudo chmod +x /root && sudo chmod +x /root/rustacki && sudo chmod +x /root/rustacki/infra && sudo chmod +x /root/rustacki/infra/assets && cd scripts
# ---------------
# ---- setup VPS
./setup.sh
```

#### step2) redeploy infra stack or services 

```bash
# ---------------
# ---- redeploy everything 
./redeploy.sh
```

#### step3) renew nginx 

```bash
# ---------------
# ---- renew nginx 
./renew.sh
```

the CI/CD approach:

...

## routes and apis

```bash
🥛 RUSTACKI WEBSOCKET STREAMING HTTP ROUTE   ==> https://event.api.your-app.app/stream
🥛 RUSTACKI WEBSOCKET STREAMING WS ROUTE     ==> wss://event.api.your-app.app/stream
🛤️ RUSTACKI HTTP APIs                        ==> https://api.your-app.app/
🛢️ RUSTACKI ADMINER                          ==> https://adminer.your-app.app
👨🏻‍💻 RUSTACKI DBEAVER                          ==> https://dbeaver.your-app.app
⛵ RUSTACKI PRTAINER                         ==> https://portainer.your-app.app
📊 RUSTACKI GRAFANA                          ==> https://grafana.your-app.app
🚥 RUSTACKI RMQ                              ==> https://rmq.your-app.app
🗞️ RUSTACKI LOGS                             ==> https://api.your-app.app/logs
🗂️ RUSTACKI ASSETS FOLDER                    ==> https://api.your-app.app/assets
```

## 🗃️ wikis, docs, erds, schemas and collections

[Rust Ownership and Borrowing Rules](https://github.com/wildonion/gvm/wiki/Ownership-and-Borrowing-Rules)

... // postman collections 