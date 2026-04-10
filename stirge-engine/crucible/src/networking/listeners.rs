// ----------------------------------------------------------------------------
// System     : Stirge Engine
// Service    : Crucible
// File       : listeners.rs
// Engineer   : Christian Westbrook
// Copyright  : (c) 2026 Christian Westbrook
// License    : Distributed under the MIT license. See the LICENSE file in the
//              repository root for details.
//
//  Abstract
//! Defines an HTTP listener for client connections.
// ----------------------------------------------------------------------------

// Third-party imports
use tracing::{instrument, error, info, debug};

use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;

use tokio_tungstenite::accept_async;
use tokio_tungstenite::WebSocketStream;

// -----------------------------------------------------------------------------
// Public Interface
// -----------------------------------------------------------------------------

/// Represents an HTTP listener for inbound client WebSocket connections.
#[derive(Debug)]
pub struct ClientListener {
    addr: String,
    client_conn_queue_tx: Sender<WebSocketStream<TcpStream>>
}

impl ClientListener {

    /// Get a new ClientListener
    pub fn new(addr: &str, client_conn_queue_tx: Sender<WebSocketStream<TcpStream>>,) -> Self {
        Self { 
            addr: addr.to_string(),
            client_conn_queue_tx
         }
    }

    /// Start listening
    #[instrument]
    pub async fn start_listening(&self) {

        debug!("Initializing ClientListener at {}", self.addr);

        let listener = match TcpListener::bind(&self.addr).await {
            Ok(listener) => listener,
            Err(err) => { panic!("Failed to bind to address: {}", err) }
        };

        match listener.local_addr() {
            Ok(address) => {
                info!("Listening on {}", format!("{}:{}", address.ip(), address.port()));
            },
            Err(err) => {
                error!("Failed to retrieve the listening address from a TcpListener: {}", err)
            }
        };

        while let Ok((stream, _)) = listener.accept().await {
            info!("Accepted a client connection");

            let sender = self.client_conn_queue_tx.clone();

            tokio::spawn(async move {
                match accept_async(stream).await {
                    Ok(websocket_stream) => { 
                        sender.send(websocket_stream).await
                            .inspect_err(|error| error!("Unable to transmit a newly connected player's WebSocket to the simulation: {}", error))
                            .ok();
                    },
                    Err(error) => {
                        error!("Failed to create a new WebSocket stream from the provided TCP stream: {}", error); 
                    }
                };
            });
        }
    }
}