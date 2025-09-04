# IdeaNet - Backend API Specification v2 (Graph-Native)

## 1. Overview

IdeaNet's backend provides a graph-native API for node-based knowledge management with vector search and flexible data storage. Built with Axum (Rust) and SurrealDB's multi-model capabilities, the API focuses purely on data persistence, retrieval, and semantic search while remaining completely agnostic about how data is used or rendered.

### Backend Architecture
The IdeaNet backend leverages SurrealDB's graph, document, and vector models:
- **Axum** for HTTP server and middleware
- **SurrealDB** for graph database with embedded vectors
- **JWT** for stateless authentication (OAuth ready for future)
- **Vector Search** for AI-powered content discovery
- **Flexible Schemas** for diverse node data types

### Backend Modules:
1. **`ideanet-core-api`** - Authentication and project management
2. **`ideanet-graph-api`** - Node and connection management
3. **`ideanet-vector-api`** - Embedding and semantic search
4. **`ideanet-files-api`** - File upload and storage management

*Note: Frontend UX and interaction patterns are detailed in [IdeaNet Frontend Specification v2](./ideanet-frontend-v2.md).*

## 2. Executive Summary

The IdeaNet backend provides a knowledge graph platform where ideas are represented as connected nodes with flexible data storage. The backend focuses exclusively on data persistence, retrieval, and processing while remaining completely agnostic about how nodes are rendered or what functionality they provide.

### Key Backend Capabilities
- **Graph-Native Storage**: Nodes and connections as first-class entities
- **Vector-Powered Intelligence**: Semantic search and AI suggestions
- **Flexible Node Data**: Schema-less JSON storage for any node type
- **OAuth-Ready Architecture**: Prepared for Microsoft, Google, Apple integration
- **Pure Data API**: No assumptions about frontend implementation

## 3. Technology Stack

### Core Technologies
- **Axum 0.7** - HTTP framework for REST API
- **SurrealDB 1.0** - Multi-model database (graph + vector + document)
- **serde** - JSON serialization/deserialization
- **JWT + OAuth** - Authentication (future: Microsoft, Google, Apple)
- **tokio** - Async runtime

### Deployment Architecture
```
┌─────────────────────────────────────────┐
│         Graph-Native Backend            │
│  ┌─────────────────────────────────────┐│
│  │      Axum HTTP Server               ││
│  │   - REST API endpoints              ││
│  │   - JWT + OAuth middleware          ││
│  │   - Vector search & AI integration  ││
│  └─────────────────────────────────────┘│
│  ┌─────────────────────────────────────┐│
│  │      SurrealDB Multi-Model          ││
│  │   - Graph: nodes & connections      ││
│  │   - Document: flexible node data    ││
│  │   - Vector: semantic embeddings     ││
│  └─────────────────────────────────────┘│
│  ┌─────────────────────────────────────┐│
│  │      Storage Systems                ││
│  │   - Node data & metadata           ││
│  │   - Audio/media files               ││
│  │   - Vector embedding cache          ││
│  └─────────────────────────────────────┘│
└─────────────────────────────────────────┘
```

---

## 4. Core API Endpoints

### 4.1 Authentication

#### POST `/api/auth/register`
Create a new user account.

**Request:**
```json
{
  "email": "user@example.com",
  "password": "securePassword123",
  "display_name": "John Doe"
}
```

**Response (201):**
```json
{
  "user": {
    "id": "users:uuid",
    "email": "user@example.com",
    "display_name": "John Doe",
    "auth_providers": ["local"],
    "created_at": "2025-09-05T10:00:00Z"
  },
  "token": "jwt-access-token"
}
```

#### POST `/api/auth/login`
Authenticate user and receive access token.

#### POST `/api/auth/oauth/{provider}` (Future)
OAuth integration endpoints (currently returns 501 Not Implemented).
- Supported providers: `microsoft`, `google`, `apple`

### 4.2 Projects (Graph Containers)

#### GET `/api/projects`
List all projects for authenticated user.

#### POST `/api/projects`
Create a new project.

**Request:**
```json
{
  "name": "My Knowledge Graph",
  "description": "A graph about machine learning concepts",
  "graph_settings": {
    "default_layout": "force-directed",
    "viewport": {"x": 0, "y": 0, "zoom": 1.0}
  }
}
```

#### GET `/api/projects/{id}/graph`
Get complete graph structure for a project.

**Response:**
```json
{
  "project": {
    "id": "projects:uuid",
    "name": "My Knowledge Graph",
    "description": "A graph about machine learning concepts",
    "graph_settings": {
      "default_layout": "force-directed",
      "viewport": {"x": 0, "y": 0, "zoom": 1.0}
    },
    "created_at": "2025-09-05T10:00:00Z",
    "updated_at": "2025-09-05T12:30:00Z"
  },
  "nodes": [
    {
      "id": "nodes:uuid1",
      "node_type": "text",
      "position": {"x": 100, "y": 200},
      "size": {"width": 300, "height": 150},
      "data": {
        "content": "Neural networks are computational models...",
        "formatting": {"bold": [0, 15]}
      },
      "created_at": "2025-09-05T10:15:00Z",
      "updated_at": "2025-09-05T11:00:00Z"
    },
    {
      "id": "nodes:uuid2",
      "node_type": "audio", 
      "position": {"x": 500, "y": 300},
      "size": {"width": 400, "height": 200},
      "data": {
        "file_id": "files:audio-uuid",
        "transcript": "In this recording, I explain deep learning...",
        "duration_ms": 45000
      },
      "created_at": "2025-09-05T10:30:00Z",
      "updated_at": "2025-09-05T10:30:00Z"
    }
  ],
  "connections": [
    {
      "id": "connects:uuid",
      "from": "nodes:uuid1",
      "to": "nodes:uuid2",
      "connection_type": "elaborates",
      "weight": 0.8,
      "metadata": {
        "description": "Audio elaborates on text concept",
        "auto_generated": false
      },
      "created_at": "2025-09-05T10:45:00Z"
    }
  ]
}
```

### 4.3 Node Management

#### POST `/api/projects/{project_id}/nodes`
Create a new node in the graph.

**Request:**
```json
{
  "node_type": "audio",
  "position": {"x": 300, "y": 400},
  "size": {"width": 400, "height": 200},
  "data": {
    "title": "Recording about transformers"
  }
}
```

**Response (201):**
```json
{
  "id": "nodes:new-uuid",
  "node_type": "audio",
  "position": {"x": 300, "y": 400},
  "size": {"width": 400, "height": 200},
  "data": {
    "title": "Recording about transformers"
  },
  "created_at": "2025-09-05T13:00:00Z",
  "updated_at": "2025-09-05T13:00:00Z"
}
```

#### PUT `/api/nodes/{id}`
Update node data, position, or size.

#### DELETE `/api/nodes/{id}`
Delete node and clean up connections.

#### GET `/api/nodes/{id}/connections`
Get all connections for a specific node.

**Response:**
```json
{
  "incoming": [
    {
      "id": "connects:uuid1",
      "from": "nodes:other-uuid",
      "connection_type": "dependency",
      "weight": 0.9,
      "metadata": {"description": "Depends on this concept"}
    }
  ],
  "outgoing": [
    {
      "id": "connects:uuid2", 
      "to": "nodes:another-uuid",
      "connection_type": "reference",
      "weight": 0.7,
      "metadata": {"description": "References this idea"}
    }
  ]
}
```

### 4.4 Graph Connections

#### POST `/api/connections`
Create connection between nodes.

**Request:**
```json
{
  "from_node": "nodes:uuid1",
  "to_node": "nodes:uuid2",
  "connection_type": "sequence",
  "weight": 0.8,
  "metadata": {
    "description": "Step 2 follows step 1",
    "auto_generated": false
  }
}
```

#### PUT `/api/connections/{id}`
Update connection weight or metadata.

#### DELETE `/api/connections/{id}`
Remove connection between nodes.

#### GET `/api/graph/traverse`
Execute graph traversal queries.

**Query Parameters:**
- `start_node`: Starting node ID
- `max_depth`: Maximum traversal depth (default: 3)
- `connection_types`: Comma-separated connection types to follow
- `node_types`: Comma-separated node types to include

**Example:**
`GET /api/graph/traverse?start_node=nodes:uuid1&max_depth=2&connection_types=reference,elaborates`

**Response:**
```json
{
  "paths": [
    {
      "nodes": ["nodes:uuid1", "nodes:uuid2", "nodes:uuid3"],
      "connections": ["connects:uuid1", "connects:uuid2"],
      "path_strength": 0.75
    }
  ],
  "subgraph": {
    "nodes": [
      {"id": "nodes:uuid1", "node_type": "text", "data": {...}},
      {"id": "nodes:uuid2", "node_type": "audio", "data": {...}}
    ],
    "connections": [
      {"id": "connects:uuid1", "from": "nodes:uuid1", "to": "nodes:uuid2"}
    ]
  }
}
```

### 4.5 Vector Search & AI

#### POST `/api/search/semantic`
Semantic search using vector embeddings.

**Request:**
```json
{
  "query": "machine learning algorithms",
  "search_types": ["nodes", "files"],
  "project_ids": ["projects:uuid1"],
  "limit": 20,
  "min_similarity": 0.7
}
```

**Response:**
```json
{
  "results": [
    {
      "item_id": "nodes:uuid",
      "item_type": "node",
      "similarity": 0.92,
      "snippet": "Deep learning neural networks for classification...",
      "node_type": "text",
      "project_id": "projects:uuid1"
    },
    {
      "item_id": "files:uuid",
      "item_type": "file",
      "similarity": 0.87,
      "filename": "ml-research-paper.pdf",
      "project_id": "projects:uuid1"
    }
  ],
  "total_results": 15,
  "query_embedding_time_ms": 45
}
```

#### GET `/api/nodes/{id}/similar`
Find nodes similar to the given node.

**Query Parameters:**
- `limit`: Maximum results (default: 10)
- `min_similarity`: Minimum similarity score (default: 0.6)
- `same_project_only`: Limit to same project (default: true)

#### POST `/api/nodes/{id}/suggest-connections`
Get AI-powered connection suggestions for a node.

**Response:**
```json
{
  "suggestions": [
    {
      "target_node": "nodes:other-uuid",
      "suggested_type": "elaborates",
      "confidence": 0.85,
      "reasoning": "Both nodes discuss neural network architectures"
    }
  ]
}
```

---

## 4.6 File Management

#### POST `/api/files`
Upload a file.

**Request:** `multipart/form-data`
- `file`: File data
- `project_id`: Project ID (optional)
- `description`: File description (optional)

**Response (201):**
```json
{
  "id": "files:uuid",
  "filename": "recording.mp3",
  "original_filename": "My Recording.mp3",
  "file_size": 1048576,
  "mime_type": "audio/mpeg",
  "project_id": "projects:uuid",
  "download_url": "/api/files/files:uuid/download",
  "embeddings_generated": false,
  "created_at": "2025-09-05T14:00:00Z"
}
```

#### GET `/api/files/{id}/download`
Download a file.

#### DELETE `/api/files/{id}`
Delete a file and cleanup storage.

---

## 5. Database Schema (SurrealDB)

### Core Tables

```sql
-- Users with OAuth provider support
DEFINE TABLE users SCHEMAFULL;
DEFINE FIELD email ON users TYPE string;
DEFINE FIELD password_hash ON users TYPE option<string>;
DEFINE FIELD display_name ON users TYPE string;
DEFINE FIELD auth_providers ON users TYPE array<string> DEFAULT ["local"];
DEFINE FIELD provider_data ON users TYPE object DEFAULT {};
DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now();
DEFINE INDEX users_email ON users COLUMNS email UNIQUE;

-- Projects as graph containers
DEFINE TABLE projects SCHEMAFULL;
DEFINE FIELD name ON projects TYPE string;
DEFINE FIELD description ON projects TYPE string;
DEFINE FIELD owner ON projects TYPE record<users>;
DEFINE FIELD graph_settings ON projects TYPE object;
DEFINE FIELD created_at ON projects TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON projects TYPE datetime DEFAULT time::now();

-- Nodes with flexible data structure (no plugin references)
DEFINE TABLE nodes SCHEMAFULL;
DEFINE FIELD project ON nodes TYPE record<projects>;
DEFINE FIELD node_type ON nodes TYPE string;
DEFINE FIELD position ON nodes TYPE object;  -- {x: float, y: float}
DEFINE FIELD size ON nodes TYPE object;      -- {width: float, height: float}
DEFINE FIELD data ON nodes TYPE object;      -- Completely flexible JSON data
DEFINE FIELD embeddings ON nodes TYPE array<float>;
DEFINE FIELD embeddings_hash ON nodes TYPE string;
DEFINE FIELD created_at ON nodes TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON nodes TYPE datetime DEFAULT time::now();
DEFINE FIELD created_by ON nodes TYPE record<users>;

-- Graph connections using SurrealDB's native RELATES
DEFINE TABLE connects RELATES nodes TO nodes SCHEMAFULL;
DEFINE FIELD connection_type ON connects TYPE string;
DEFINE FIELD weight ON connects TYPE float DEFAULT 1.0;
DEFINE FIELD metadata ON connects TYPE object;
DEFINE FIELD auto_generated ON connects TYPE bool DEFAULT false;
DEFINE FIELD created_at ON connects TYPE datetime DEFAULT time::now();
DEFINE FIELD created_by ON connects TYPE record<users>;

-- Files with vector embeddings
DEFINE TABLE files SCHEMAFULL;
DEFINE FIELD filename ON files TYPE string;
DEFINE FIELD original_filename ON files TYPE string;
DEFINE FIELD file_path ON files TYPE string;
DEFINE FIELD file_size ON files TYPE int;
DEFINE FIELD mime_type ON files TYPE string;
DEFINE FIELD owner ON files TYPE record<users>;
DEFINE FIELD project ON files TYPE option<record<projects>>;
DEFINE FIELD embeddings ON files TYPE array<float>;
DEFINE FIELD embeddings_generated ON files TYPE bool DEFAULT false;
DEFINE FIELD created_at ON files TYPE datetime DEFAULT time::now();

-- Performance indexes
DEFINE INDEX nodes_project ON nodes COLUMNS project;
DEFINE INDEX nodes_type ON nodes COLUMNS node_type;
DEFINE INDEX nodes_embeddings ON nodes COLUMNS embeddings MTREE DIMENSION 1536;
DEFINE INDEX files_embeddings ON files COLUMNS embeddings MTREE DIMENSION 1536;
DEFINE INDEX connects_type ON connects COLUMNS connection_type;
DEFINE INDEX connects_weight ON connects COLUMNS weight;
```

### Example Queries

```sql
-- Get complete project graph
SELECT *, 
  (SELECT * FROM nodes WHERE project = $parent.id) AS nodes,
  (SELECT * FROM connects WHERE in.project = $parent.id OR out.project = $parent.id) AS connections
FROM projects:project_id;

-- Find nodes similar to a given node
SELECT *, vector::similarity::cosine(embeddings, $query_embedding) AS similarity
FROM nodes 
WHERE project = $project_id 
  AND similarity > 0.7 
  AND id != $source_node_id
ORDER BY similarity DESC 
LIMIT 10;

-- Graph traversal from a starting node
SELECT * FROM nodes:start_id <-connects<-nodes<-connects*..3 nodes
WHERE connection_type IN ["reference", "elaborates"];

-- Get nodes with their connection counts
SELECT *, 
  count(->connects) AS outgoing_count,
  count(<-connects) AS incoming_count
FROM nodes 
WHERE project = projects:project_id;
```

---

## 6. User Stories & Implementation Plan

### Phase 1: Graph Foundation (Weeks 1-4)
- **API-001**: JWT authentication with OAuth architecture
- **API-002**: User management and profiles
- **API-003**: Project CRUD as graph containers
- **API-004**: Basic node CRUD operations
- **API-005**: Connection management
- **Deliverable**: Working graph creation and basic navigation

### Phase 2: Core Node Types (Weeks 5-8)
- **API-006**: Plugin registry foundation
- **API-007**: File upload with metadata
- **API-008**: Core plugins (text, audio, file nodes)
- **API-009**: Basic graph queries and traversal
- **Deliverable**: Functional node types with plugin system

### Phase 3: Vector Intelligence (Weeks 9-12)
- **API-010**: Embedding generation pipeline
- **API-011**: Vector similarity search
- **API-012**: AI-powered recommendations
- **API-013**: Semantic discovery features
- **Deliverable**: AI-powered content discovery

**Future Enhancements:**
- 🔮 OAuth integration (Microsoft, Google, Apple)
- 🔮 Real-time collaborative graph editing
- 🔮 Advanced graph analytics and clustering
- 🔮 Multi-modal AI analysis (image, video, code)

---

## 7. API Design Principles

### Pure Data Focus
- **No UI Logic**: Backend handles only data persistence and retrieval
- **Frontend Agnostic**: API can support web, desktop, mobile, or CLI clients
- **Clean Separation**: No node focus, expansion, or interaction concerns in API

### Graph-First Design
- **Node-Centric URLs**: `/api/nodes/{id}`, `/api/nodes/{id}/connections`
- **Relationship Emphasis**: Connections are first-class entities
- **Traversal Support**: Rich graph query capabilities
- **Semantic Search**: Vector-powered discovery across graph

### Security & Performance
- **JWT Authentication**: Stateless with OAuth extensibility
- **Plugin Sandboxing**: WASM-based secure execution
- **Vector Indexing**: Optimized similarity search
- **Resource Limiting**: Plugin execution constraints

This backend API specification focuses purely on data management and processing, leaving all UX concerns to the frontend specification. The graph-native approach with SurrealDB provides a solid foundation for the knowledge management platform while maintaining clean architectural boundaries.
