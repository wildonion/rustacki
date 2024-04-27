


// audio/video/chat market streamer over P2P, TCP, gRPC bidi, webRTC, MPEG-DASH
// audio/video/chat codec to minimize the size during sending the packets to the client
// see rustacki.spec and projects
// ws streaming: `api/ws/hoop.rs` and `actors/ws/servers/hoop.rs` and `actors/ws/sessions/hoop.rs` for both directions event streaming
// tcp streaming: server/tcp/mod.rs
// streaming over an authenticated channel and a consensus mechanism between streamer nodes 
// and peers in a cluster to talk to each other using p2p gossipsub behind a dlm or a distributed
// lock manager lile redlock, k8 and zookeeper (lockers/dlm.rs)
// use secure_session for tcp connection

// https://www.cloudflare.com/learning/video/what-is-mpeg-dash/
// httsp://agora.io

pub mod grpc;
pub mod webrtc;
pub mod tcp;
pub mod p2p;