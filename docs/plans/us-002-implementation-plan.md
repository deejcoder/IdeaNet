# IdeaNet US-002 Implementation Plan - Database Infrastructure

**Version:** 1.0  
**Date:** September 4, 2025  
**User Story:** US-002 - Configure database foundation (Phase 2)  
**Dependencies:** US-001 foundation must be complete  

## Executive Summary

This plan implements US-002: essential database infrastructure for IdeaNet. Building on the basic connectivity established in US-001, this phase adds schema migrations, improved configuration management, enhanced error handling, and basic transaction support - the core foundation needed for business logic implementation in Phase 3.

## User Story Reference

**US-002**: Configure database foundation (Phase 2)
- **Description**: As a developer, I want SurrealDB properly configured with schema migration system and robust connection management so that data persistence is reliable and can evolve with the application
- **Acceptance criteria**:
  - Database schema migration system for version updates and rollbacks
  - Enhanced configuration management with environment detection
  - Improved error handling and recovery patterns
  - Basic transaction management for atomic operations
  - Database schema validation and integrity checks
  - Foundation ready for business entity implementation in Phase 3

## Scope and Objectives

### ✅ What MUST be implemented

#### 1. Schema Migration System
- Database versioning and migration framework
- Migration file management and execution
- Basic rollback capabilities for failed migrations
- Migration history tracking
- Schema validation and integrity checks

#### 2. Enhanced Configuration Management
- Single configuration file with environment detection
- User-customizable database paths
- Configuration validation and sensible defaults
- Platform-specific storage path resolution

#### 3. Improved Error Handling
- Enhanced error types with better context
- Proper error propagation from database to frontend
- Connection failure recovery patterns
- Graceful degradation strategies

#### 4. Basic Transaction Management
- Simple transaction wrapper for atomic operations
- Transaction rollback on errors
- Foundation for business logic transactions

### 🚫 What remains out of scope

- ❌ Business entities or domain models (Phase 3)
- ❌ User authentication or authorization (Future)
- ❌ Plugin system or event bus (Phase 7)
- ❌ AI, audio, or other module features (Phase 5+)
- ❌ Connection pooling (not needed for embedded SurrealDB)
- ❌ Automated backup scheduling (premature - defer to Phase 6)
- ❌ Performance monitoring infrastructure (premature optimization)
- ❌ Complex admin UI panels (not user-facing features)
- ❌ Multi-environment deployment configs (desktop app, not server)
- ❌ Advanced database features (sharding, replication)
- ❌ Real-time synchronization

## Architecture Design

### Enhanced Database Layer Structure

```
┌─────────────────────────────────────────┐
│         Frontend (React + TS)           │ ← Enhanced DB status
├─────────────────────────────────────────┤
│        Tauri Commands Layer             │ ← Database and migration commands
├─────────────────────────────────────────┤
│       Application Services              │ ← Enhanced DatabaseService
│  ┌─────────────────────────────────────┐ │
│  │ DatabaseService                     │ │ ← Core database operations
│  │ MigrationService                    │ │ ← Schema migrations
│  │ ConfigurationService                │ │ ← Configuration management
│  └─────────────────────────────────────┘ │
├─────────────────────────────────────────┤
│          Domain Layer                   │ ← Enhanced error types, models
├─────────────────────────────────────────┤
│       Infrastructure Layer              │ ← Enhanced SurrealAdapter
│  ┌─────────────────────────────────────┐ │
│  │ SurrealAdapter                      │ │ ← Database adapter with transactions
│  │ MigrationRunner                     │ │ ← Migration execution
│  │ ConfigurationLoader                 │ │ ← Config loading and validation
│  └─────────────────────────────────────┘ │
└─────────────────────────────────────────┘
```

### Project Structure Updates

```
src-tauri/
├── Cargo.toml                     # Updated dependencies
├── src/
│   ├── main.rs                    # Enhanced initialization
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── database.rs            # Enhanced database commands
│   │   └── migration.rs           # Migration commands
│   ├── application/
│   │   ├── mod.rs
│   │   ├── ports/
│   │   │   ├── mod.rs
│   │   │   ├── database.rs        # Enhanced DatabasePort
│   │   │   └── migration.rs       # MigrationPort
│   │   └── services/
│   │       ├── mod.rs
│   │       ├── database.rs        # Enhanced DatabaseService
│   │       ├── migration.rs       # MigrationService
│   │       └── configuration.rs   # ConfigurationService
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── errors.rs              # Enhanced error types
│   │   └── models/
│   │       ├── mod.rs
│   │       ├── configuration.rs   # Config models
│   │       └── migration.rs       # Migration models
│   └── infrastructure/
│       ├── mod.rs
│       ├── database/
│       │   ├── mod.rs
│       │   ├── adapter.rs         # Enhanced SurrealAdapter with transactions
│       │   ├── migrations.rs      # Migration execution
│       │   └── config.rs          # Configuration loading and validation
├── migrations/                    # Migration files
│   ├── 001_initial_schema.surql
│   └── migration_manifest.json
└── ideanet.toml                   # Single configuration file

src/                               # React frontend updates
├── components/
│   └── DatabaseStatus.tsx         # Enhanced with migration status
└── hooks/
    ├── useDatabase.ts             # Enhanced database operations
    └── useMigration.ts            # Migration operations
```

## Implementation Details

### Enhanced Dependencies

```toml
[dependencies]
# Existing dependencies...
tauri = { version = "2", features = ["rustls-tls"] }
surrealdb = { version = "1.0", features = ["kv-rocksdb"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"
async-trait = "0.1"

# New dependencies for US-002
chrono = { version = "0.4", features = ["serde"] }
toml = "0.8"
sha2 = "0.10"  # For migration checksums
semver = "1.0"  # For version management
dirs = "5.0"  # For platform-specific paths
tracing = "0.1"  # For better logging
```

### Component Implementation Details

#### 1. Schema Migration System

**Migration File Format:**
```sql
-- migrations/001_initial_schema.surql
-- Migration: Initial Schema
-- Version: 0.1.0
-- Author: System
-- Date: 2025-09-04

-- Create initial tables with basic structure
DEFINE TABLE migration_history SCHEMAFULL;
DEFINE FIELD version ON migration_history TYPE string;
DEFINE FIELD applied_at ON migration_history TYPE datetime;
DEFINE FIELD checksum ON migration_history TYPE string;
DEFINE FIELD success ON migration_history TYPE bool;
```

**Migration Manifest:**
```json
{
  "version": "1.0.0",
  "migrations": [
    {
      "id": "001",
      "name": "initial_schema",
      "version": "0.1.0",
      "checksum": "sha256:...",
      "dependencies": [],
      "rollback_available": true
    }
  ]
}
```

#### 2. Enhanced Transaction Management

**Transaction Configuration:**
```rust
pub struct TransactionConfig {
    pub timeout: Duration,
    pub retry_attempts: u32,
    pub retry_delay: Duration,
}

impl Default for TransactionConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            retry_attempts: 3,
            retry_delay: Duration::from_millis(100),
        }
    }
}
```

#### 3. Configuration Management

**Single Configuration File (ideanet.toml):**

```toml
[database]
engine = "rocksdb"
# Path will be auto-detected: debug builds use ./data, release uses app data dir
namespace = "ideanet"
database = "main"

[migrations]
auto_run = true  # Run pending migrations on startup
validate_checksums = true

[transactions]
timeout_seconds = 30
retry_attempts = 3
retry_delay_ms = 100

[logging]
level = "info"  # debug, info, warn, error
log_queries = false  # Only enable for debugging
```

**Configuration Loading:**
```rust
pub struct DatabaseConfig {
    pub engine: String,
    pub path: PathBuf,  // Auto-resolved based on environment
    pub namespace: String,
    pub database: String,
    pub migrations: MigrationConfig,
    pub transactions: TransactionConfig,
    pub logging: LoggingConfig,
}

impl DatabaseConfig {
    pub fn load() -> Result<Self, ConfigError> {
        // Load from ideanet.toml with sensible defaults
        // Auto-detect paths based on debug/release mode
    }
}
```

#### 4. Enhanced Error Handling

**Error Types:**
```rust
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Migration failed: {0}")]
    MigrationFailed(String),
    
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Schema validation failed: {0}")]
    SchemaValidationFailed(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] surrealdb::Error),
}
```

**Error Context and Recovery:**
- Detailed error messages with context
- Automatic retry logic for transient failures
- Graceful degradation strategies
- Proper error propagation to frontend

#### 5. Basic Transaction Support

**Transaction Wrapper:**
```rust
pub async fn with_transaction<F, R>(&self, operation: F) -> Result<R, DatabaseError>
where
    F: FnOnce(&mut surrealdb::Transaction) -> BoxFuture<'_, Result<R, DatabaseError>>,
{
    let mut tx = self.db.begin().await
        .map_err(|e| DatabaseError::TransactionFailed(e.to_string()))?;
    
    match operation(&mut tx).await {
        Ok(result) => {
            tx.commit().await
                .map_err(|e| DatabaseError::TransactionFailed(e.to_string()))?;
            Ok(result)
        }
        Err(error) => {
            if let Err(rollback_error) = tx.cancel().await {
                tracing::error!("Failed to rollback transaction: {}", rollback_error);
            }
            Err(error)
        }
    }
}
```

## Frontend Enhancements

### Enhanced Database Status Component

```typescript
interface DatabaseStatusProps {
  showMigrationStatus?: boolean;
}

interface DatabaseStatus {
  connected: boolean;
  version: string;
  migrations: {
    current: string;
    pending: number;
    status: 'up-to-date' | 'pending' | 'error';
  };
  lastError?: string;
}
```

### Migration Status Interface

```typescript
interface MigrationStatus {
  current: string;
  available: string;
  pending: string[];
  canMigrate: boolean;
  isRunning: boolean;
}

interface UseMigrationHook {
  status: MigrationStatus;
  runPendingMigrations: () => Promise<void>;
  refreshStatus: () => Promise<void>;
}
```

## Implementation Timeline

### Week 1: Core Infrastructure
- **Day 1-2**: Enhanced database adapter with transaction support
- **Day 3-4**: Configuration management system with auto path detection
- **Day 5**: Improved error handling and logging

### Week 2: Migration System and Testing
- **Day 1-2**: Migration framework and runner implementation
- **Day 3**: Migration validation and basic rollback
- **Day 4**: Frontend integration and status display
- **Day 5**: Unit and integration testing, documentation

## Success Criteria

### Functional Requirements
- [ ] Database connections work reliably with proper error handling
- [ ] Schema migrations apply and rollback successfully
- [ ] Configuration loads automatically with sensible defaults
- [ ] Basic transactions support atomic operations
- [ ] Migration status visible in UI
- [ ] Foundation ready for business entities in Phase 3

### Quality Requirements
- [ ] Core functionality has unit test coverage
- [ ] Integration tests validate migration workflows
- [ ] Error handling provides clear, actionable messages
- [ ] Code follows Rust best practices and conventions
- [ ] Documentation covers setup and usage

### Performance Targets
- [ ] Database initialization: < 1s
- [ ] Migration application: < 5s for typical migrations
- [ ] Memory usage: reasonable for desktop application
- [ ] No performance regression from Phase 1

## Risk Mitigation

### Technical Risks
1. **Migration Failures**: Checksum validation and basic rollback capability
2. **Configuration Issues**: Sensible defaults and validation
3. **Data Corruption**: Transaction integrity and proper error handling
4. **Platform Compatibility**: Use standard Rust/Tauri patterns

### Implementation Risks
1. **Scope Creep**: Stick to essential features only
2. **Over-Engineering**: Keep solutions simple and focused
3. **Migration Complexity**: Start with simple file-based migrations
4. **Testing Overhead**: Focus on critical path testing

## Testing Strategy

### Unit Tests
- Migration execution and rollback
- Configuration loading and validation
- Transaction wrapper functionality
- Error handling and recovery

### Integration Tests
- End-to-end migration workflows
- Database initialization and connection
- Configuration loading in different environments
- Error scenarios and recovery patterns

### Manual Testing
- UI migration status display
- Error message clarity
- Cross-platform path resolution
- Database file creation and access

## Future Considerations

### Phase 3 Preparation
- Database schema ready for business entities
- Migration system supports business logic changes
- Transaction patterns ready for complex operations
- Error handling patterns established

### Potential Enhancements (Future Phases)
- Automated backup system (Phase 6+)
- Performance monitoring (when needed)
- Advanced migration features (dependencies, conditional migrations)
- Configuration UI (if user-requested)

---

**Dependencies**: US-001 foundation must be complete  
**Next Phase**: US-004 - Project management and core domain entities  
**Estimated Effort**: 2 weeks (1 developer)  
**Priority**: High - Required for all future development phases
