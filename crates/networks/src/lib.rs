// TODO: build like zig, like a graph with all the protocols

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod protocols;

pub trait NetworkManager {
    /// The type representing the internal state of the protocol.
    /// This is a placeholder for the protocol's state (e.g., a connection state or configuration).
    /// The exact type is determined by the implementation of the trait.
    type State;

    /// The type for the RPC request, which represents the data to be sent in the remote procedure call.
    /// This type must implement both Serialize and Deserialize, allowing for automatic conversion
    /// from and to byte representations for transmission over the network.
    type RpcRequest: Serialize + for<'de> Deserialize<'de>;

    /// The type for the RPC response, representing the data returned from the remote procedure call.
    /// Like RpcRequest, this type must also implement Serialize and Deserialize for proper
    /// serialization and deserialization.
    type RpcResponse: Serialize + for<'de> Deserialize<'de>;

    fn initialize(&mut self) -> Self::State;

    /// WARNING: For code style and standardization, the format for writing an RPC procedure name must follow this pattern: `protocol-action-rpc`.
    /// Examples: `http-get-rpc`, `icpm-ping-rpc`

    // TODO: Resolve this warning
    async fn call_procedure(
        &mut self,
        procedure_name: &str,
        request: &Self::RpcRequest,
    ) -> Result<Self::RpcResponse, String>;

    /// Determines if the protocol is connection-oriented.
    /// By default, this method returns `false`, meaning the protocol is not expected to establish
    /// a persistent connection (e.g., stateless protocols like HTTP).
    /// Implementations can override this to return `true` for protocols like TCP that require
    /// a persistent connection.
    ///
    /// # Returns
    /// - `bool`: `true` if the protocol requires a connection, `false` otherwise.
    fn is_connection_oriented(&self) -> bool {
        false // Default behavior: assume the protocol is not connection-oriented.
    }

    /// Closes the connection, if applicable.
    /// This is a default no-op implementation, which can be overridden by protocols that
    /// need to close or clean up connections after usage (e.g., TCP or WebSocket).
    fn close_connection(&mut self) {
        println!("close_connection not implemented for this protocol.");
        // Default behavior: prints a message indicating the method isn't implemented.
    }
}
