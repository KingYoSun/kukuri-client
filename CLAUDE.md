# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Kukuri Client is a distributed social network desktop application built with:
- **Frontend**: React + TypeScript + Vite + Tailwind CSS
- **Backend**: Tauri (Rust) with iroh P2P protocols
- **Architecture**: Local-first, distributed, no central server

## Essential Commands

### Development
```bash
# Install dependencies
pnpm install

# Start development (opens Tauri window)
pnpm tauri dev

# Frontend-only development
pnpm dev

# Build for production
pnpm tauri build
```

### Testing
```bash
# Run all frontend tests
pnpm test

# Run tests in watch mode
pnpm test:watch

# Run E2E tests
pnpm test:e2e

# Run Rust tests
cd src-tauri && cargo test

# Run Rust tests with test utilities
cd src-tauri && cargo test --features test-utils
```

### Linting & Type Checking
```bash
# TypeScript type checking
pnpm tsc --noEmit

# Rust linting
cd src-tauri && cargo clippy
```

## Architecture Overview

### Data Flow Pattern
1. **User Action** → React Component → Custom Hook → Service Layer
2. **Service Layer** → Tauri Command (via `invoke`)
3. **Rust Command Handler** → Repository → iroh Storage
4. **Response** → Back through layers to UI

### Key Architectural Decisions
- **State Management**: Zustand stores (`src/stores/`)
- **Data Validation**: Zod schemas in models
- **P2P Storage**: iroh-docs for synced data, iroh-blobs for files
- **Event System**: Document events via Tauri's event system

### Storage Architecture
- **Users**: Stored in `users` Namespace
- **Posts**: Stored in `posts` Namespace  
- **Settings**: Stored in `settings` Namespace
- **Binary Data**: Stored in iroh-blobs

### Testing Strategy
- **Unit Tests**: Components, hooks, models in `tests/unit/`
- **Integration Tests**: Full workflows in `tests/integration/`
- **E2E Tests**: User journeys in `tests/e2e/`

## Common Development Tasks

### Adding a New Feature
1. Define TypeScript model in `src/models/`
2. Create Rust model in `src-tauri/src/models/`
3. Add Tauri command in `src-tauri/src/commands/`
4. Create repository in `src-tauri/src/storage/repository/`
5. Add service in `src/services/`
6. Create store in `src/stores/`
7. Build UI components and hooks

### Working with iroh
- Documents are accessed via `IrohNode` in storage layer
- Each data type has its own Namespace
- Sync happens automatically via iroh protocols
- Binary data uses iroh-blobs for content-addressed storage

### Running Single Tests
```bash
# Frontend: Run specific test file
pnpm test src/services/storage-service.test.ts

# Rust: Run specific test
cd src-tauri && cargo test test_name
```

## Important Patterns

### Tauri Commands
Commands follow this pattern:
```rust
#[tauri::command]
pub async fn command_name(
    state: State<'_, AppState>,
    param: Type
) -> Result<ReturnType, String>
```

### Repository Pattern
Each data type has a repository module with standard CRUD operations that interact with iroh-docs.

### Event Subscriptions
Document changes emit events that frontend subscribes to via `useDocumentEvents` hook.

## Memory Bank Usage

The `memory-bank/` directory contains important project context and documentation:

### Available Context Files
- **projectbrief.md**: MVP requirements and implementation phases
- **systemPatterns.md**: Architecture patterns and data flow diagrams
- **techContext.md**: Technical decisions and implementation details
- **activeContext.md**: Current development status and active tasks
- **progress.md**: Development progress tracking
- **productContext.md**: Product vision and user experience goals

### When to Reference Memory Bank
1. **Before implementing new features**: Check projectbrief.md for alignment with MVP goals
2. **Architecture decisions**: Reference systemPatterns.md for established patterns
3. **Data flow implementation**: Follow patterns documented in systemPatterns.md
4. **Technical choices**: Consult techContext.md for rationale behind technology selections

### Memory Bank Guidelines
- Always follow the established patterns in systemPatterns.md
- Ensure new features align with the MVP scope in projectbrief.md
- Update progress.md when completing significant milestones
- Reference activeContext.md for current development priorities