# Iroh-Docs Context for AI Coding

## Overview
Iroh-Docs is a library that provides multi-dimensional key-value documents with an efficient synchronization protocol. It's part of the Iroh suite of networking libraries developed by n0-computer.

## Core Concepts

### Replicas
- A replica contains an unlimited number of entries
- Each entry is identified by a key, its author, and the replica's namespace
- Entry values are 32-byte BLAKE3 hashes of content data, along with size and timestamp information
- Content data itself is not stored or transferred through a replica

### Authentication
- Entries are signed with two keypairs:
  - Namespace key: A token of write capability; the public key is the NamespaceId, which serves as a unique identifier for a replica
  - Author key: Proof of authorship; any number of authors may be created, and their semantic meaning is application-specific

### Synchronization
- Replicas can be synchronized between peers by exchanging messages
- Uses "range-based set reconciliation" algorithm (based on Aljoscha Meyer's paper: https://arxiv.org/abs/2212.13567)
- Efficiently computes the union of two sets over a network by recursively partitioning sets and comparing fingerprints

## Storage
- Generic storage interface with in-memory and persistent, file-based implementations
- The persistent implementation uses `redb`, an embedded key-value store
- In-memory mode uses a Vec<u8> backend
- Persistent mode stores everything in a single file on disk

## Integration with Iroh

Iroh-Docs is designed to work with the broader Iroh networking library:

- Iroh provides direct connections between peers for sending sync messages and transferring data
- Uses Iroh's Router and Endpoint for connection handling
- Iroh-Docs depends on two other Iroh protocols:
  - iroh-blobs: For content-addressed binary data storage
  - iroh-gossip: For peer discovery and message distribution

## Basic Usage Example

```rust
use iroh::{protocol::Router, Endpoint};
use iroh_blobs::{net_protocol::Blobs, util::local_pool::LocalPool, ALPN as BLOBS_ALPN};
use iroh_docs::{protocol::Docs, ALPN as DOCS_ALPN};
use iroh_gossip::{net::Gossip, ALPN as GOSSIP_ALPN};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create an iroh endpoint that includes the standard discovery mechanisms
    let endpoint = Endpoint::builder().discovery_n0().bind().await?;

    // Create a router builder
    let builder = Router::builder(endpoint);

    // Build the blobs protocol
    let blobs = Blobs::memory().build(builder.endpoint());

    // Build the gossip protocol
    let gossip = Gossip::builder().spawn(builder.endpoint().clone()).await?;

    // Build the docs protocol
    let docs = Docs::memory().spawn(&blobs, &gossip).await?;

    // Setup router
    let router = builder
        .accept(BLOBS_ALPN, blobs)
        .accept(GOSSIP_ALPN, gossip)
        .accept(DOCS_ALPN, docs)
        .spawn()
        .await?;

    // Use docs to create, manipulate, and sync documents
    Ok(())
}
```

## Key Components

### Docs
- Main entry point to the protocol
- Wraps the Engine that powers the protocol
- Can be created in memory or with persistent storage

### Engine
- Core functionality for working with documents
- Handles replication, synchronization, and network communication

### Store
- Interface for document storage
- Available in both in-memory and persistent variants

## API Guidelines

When implementing code with Iroh-Docs:

1. Start by creating a Docs instance (in-memory or persistent)
2. Set up the required Blobs and Gossip protocols
3. Use the Router to handle connections between peers
4. Create, manipulate, and synchronize documents using the Docs API
5. Use proper error handling for network operations

## Recent Updates (As of v0.33.0, February 2025)
- Upgraded to latest iroh, iroh-gossip, and iroh-blobs dependencies
- Removed individual repo project tracking
- API stabilization
- Performance improvements

## Licensing
- Dual-licensed under Apache License 2.0 and MIT License
- Contributions are welcome under the same dual-license terms

## Related Projects
- iroh: Main networking library for peer connections
- iroh-blobs: Content-addressed storage for binary data
- iroh-gossip: Peer discovery and message distribution
