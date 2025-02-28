// Copyright 2018 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.use futures::StreamExt;

use futures::future::Either;
use libp2p_mdns::Event;
use libp2p_mdns::{async_io::Behaviour, Config};
use libp2p_swarm::{Swarm, SwarmEvent};
use libp2p_swarm_test::SwarmExt as _;
use std::time::Duration;

#[async_std::test]
async fn test_discovery_async_std_ipv4() {
    env_logger::try_init().ok();

    run_discovery_test(Config::default()).await
}

#[async_std::test]
async fn test_discovery_async_std_ipv6() {
    env_logger::try_init().ok();

    let config = Config {
        enable_ipv6: true,
        ..Default::default()
    };
    run_discovery_test(config).await
}

#[async_std::test]
async fn test_expired_async_std() {
    env_logger::try_init().ok();

    let config = Config {
        ttl: Duration::from_secs(1),
        query_interval: Duration::from_secs(10),
        ..Default::default()
    };

    let mut a = create_swarm(config.clone()).await;
    let a_peer_id = *a.local_peer_id();

    let mut b = create_swarm(config).await;
    let b_peer_id = *b.local_peer_id();

    loop {
        match futures::future::select(a.next_behaviour_event(), b.next_behaviour_event()).await {
            Either::Left((Event::Expired(mut peers), _)) => {
                if peers.any(|(p, _)| p == b_peer_id) {
                    return;
                }
            }
            Either::Right((Event::Expired(mut peers), _)) => {
                if peers.any(|(p, _)| p == a_peer_id) {
                    return;
                }
            }
            _ => {}
        }
    }
}

#[async_std::test]
async fn test_no_expiration_on_close_async_std() {
    env_logger::try_init().ok();
    let config = Config {
        ttl: Duration::from_secs(120),
        query_interval: Duration::from_secs(10),
        ..Default::default()
    };

    let mut a = create_swarm(config.clone()).await;

    let b = create_swarm(config).await;
    let b_peer_id = *b.local_peer_id();
    async_std::task::spawn(b.loop_on_next());

    // 1. Connect via address from mDNS event
    loop {
        if let Event::Discovered(mut peers) = a.next_behaviour_event().await {
            if let Some((_, addr)) = peers.find(|(p, _)| p == &b_peer_id) {
                a.dial_and_wait(addr).await;
                break;
            }
        }
    }

    // 2. Close connection
    let _ = a.disconnect_peer_id(b_peer_id);
    a.wait(|event| {
        matches!(event, SwarmEvent::ConnectionClosed { peer_id, .. } if peer_id == b_peer_id)
            .then_some(())
    })
    .await;

    // 3. Ensure we can still dial via `PeerId`.
    a.dial(b_peer_id).unwrap();
    a.wait(|event| {
        matches!(event, SwarmEvent::ConnectionEstablished { peer_id, .. } if peer_id == b_peer_id)
            .then_some(())
    })
    .await;
}

async fn run_discovery_test(config: Config) {
    let mut a = create_swarm(config.clone()).await;
    let a_peer_id = *a.local_peer_id();

    let mut b = create_swarm(config).await;
    let b_peer_id = *b.local_peer_id();

    let mut discovered_a = false;
    let mut discovered_b = false;

    while !discovered_a && !discovered_b {
        match futures::future::select(a.next_behaviour_event(), b.next_behaviour_event()).await {
            Either::Left((Event::Discovered(mut peers), _)) => {
                if peers.any(|(p, _)| p == b_peer_id) {
                    discovered_b = true;
                }
            }
            Either::Right((Event::Discovered(mut peers), _)) => {
                if peers.any(|(p, _)| p == a_peer_id) {
                    discovered_a = true;
                }
            }
            _ => {}
        }
    }
}

async fn create_swarm(config: Config) -> Swarm<Behaviour> {
    let mut swarm =
        Swarm::new_ephemeral(|key| Behaviour::new(config, key.public().to_peer_id()).unwrap());
    swarm.listen().await;

    swarm
}
