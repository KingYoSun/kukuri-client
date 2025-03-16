# Automerge

> Automerge is a library which provides fast implementations of several different CRDTs, a compact compression format for these CRDTs, and a sync protocol for efficiently transmitting those changes over the network. The objective of the project is to support local-first applications in the same way that relational databases support server applications.

Important notes:

- Automerge provides a JSON-like data structure (a CRDT) that can be modified concurrently by different users, and merged again automatically.
- It aims to be PostgreSQL for your local-first app, providing mechanisms for persistence which allow application developers to avoid thinking about hard distributed computing problems.
- The core is implemented in Rust and exposed via FFI in JavaScript+WASM, C, and other languages.

## Repository Structure

- `./rust` - The Rust implementation and platform-specific wrappers (e.g., `automerge-wasm` for WASM API, `automerge-c` for C FFI bindings)
- `./javascript` - JavaScript library using `automerge-wasm` internally with idiomatic JavaScript interface
- `./scripts` - Maintenance scripts including CI scripts
- `./img` - Static assets for markdown files

## Main Packages

- `@automerge/automerge` - Main JavaScript package (current version: 2.2.9-alpha.5)
- `@automerge/automerge-wasm` - WASM bindings to Rust implementation (current version: 0.17.0)
- `automerge` - Rust crate

## Documentation

- [Main Documentation](https://automerge.org/automerge/automerge/)
- [Latest API Docs](https://docs.rs/automerge/latest/automerge)
- [JavaScript Docs](https://automerge.org/docs/hello/)
- [Binary Format Specification](https://automerge.org/automerge-binary-format-spec)

## Getting Started

For JavaScript applications:
```
npm install @automerge/automerge
```

For Rust developers, consider using [autosurgeon](https://github.com/automerge/autosurgeon)

## Support

- [Discord Server](https://discord.gg/HrpnPAU5zx) - For community support and discussion

## License

MIT License
