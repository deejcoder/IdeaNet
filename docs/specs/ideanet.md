# IdeaNet - Product Requirements Document

## 1. Overv### Core Modules:
1. **`ideanet-core`** - Core domain models, business logic, and database integration
2. **`ideanet-audio`** - Audio recording, processing, and transcription services
3. **`ideanet-blocks`** - Plugin-based block system for extensible content types
4. **`ideanet-ai`** - AI integration for suggestions, research, and content generation
5. **`ideanet-resources`** - File and resource management with linking strategies
6. **`ideanet-ui`** - React frontend with Chakra UI components

*Note: Each module contains its own subset of user stories detailed in separate module PRDs. Modules should be implemented independently while maintaining clear interfaces between them.*deaNet is an AI-powered idea crafting and project development platform that helps users express, organize, and evolve their ideas into actionable implementation plans. Built as a desktop application using Tauri, React, and TypeScript, it provides intelligent project management with embedded resources, AI-guided development suggestions, and rich multimedia capture including audio recording and transcription.

## 2. Executive Summary

IdeaNet transforms how creators, entrepreneurs, and teams develop ideas from initial conception to implementation. By combining rich multimedia capture (including audio recording with AI transcription), flexible block-based content creation, intelligent resource management, and AI-powered guidance, users can build comprehensive project documentation that evolves with their thinking.

### Key Differentiators
- **Plugin-based Block System**: Extensible content blocks for different media types and workflows
- **Audio-First Capture**: High-quality audio recording with AI transcription for natural idea capture
- **AI Development Assistant**: Context-aware suggestions for next steps and implementation guidance
- **Smart Resource Linking**: Flexible file management with soft/deep linking strategies
- **Project Evolution Tracking**: Visual timeline of how ideas develop and change over time

## 3. Narrative

Sarah, a product manager, has a breakthrough idea during her morning commute. She opens IdeaNet and quickly records a 3-minute audio note explaining her concept. The AI automatically transcribes it and suggests related market research questions. Back at her desk, she creates a new project, embeds the transcription into a structured idea block, and links relevant competitor analysis documents. 

As she develops the concept over the following weeks, IdeaNet's AI suggests implementation frameworks, identifies potential technical challenges, and recommends team members based on similar past projects. The block-based system lets her organize thoughts naturally - from audio brainstorms to technical specifications to market analysis - all interconnected and searchable.

When presenting to stakeholders, she exports a clean presentation view that shows the idea's evolution from initial audio capture to comprehensive business plan, demonstrating both the depth of thinking and the structured development process.

## 4. User Goals

### Primary Goals
- **Capture ideas naturally** without losing spontaneous insights
- **Develop ideas systematically** with AI guidance and structured thinking
- **Connect related concepts** across projects and time periods
- **Create implementation roadmaps** that turn ideas into actionable plans
- **Collaborate effectively** by sharing rich, multimedia project documentation

### Secondary Goals
- **Track idea evolution** to understand and improve thinking processes
- **Maintain project context** through rich resource linking and organization
- **Access information quickly** through powerful search and AI assistance
- **Present ideas professionally** with clean export and presentation modes

## 5. Business Goals

### Product Goals
- **Create differentiated IP platform** in the idea management space
- **Build sustainable competitive moat** through AI integration and user data
- **Establish platform ecosystem** for third-party block plugins and integrations
- **Generate recurring revenue** through AI service usage and premium features

### Market Goals
- **Capture knowledge worker market** (product managers, designers, researchers, entrepreneurs)
- **Expand to enterprise teams** with collaboration and administration features
- **Partner with AI providers** to offer specialized domain knowledge
- **Build community ecosystem** around plugin development and templates

## 6. System Architecture

### Core Modules
1. **`ideanet-core`** - Core domain models and business logic
2. **`ideanet-audio`** - Audio recording, processing, and transcription services
3. **`ideanet-blocks`** - Plugin-based block system for extensible content types
4. **`ideanet-ai`** - AI integration for suggestions, research, and content generation
5. **`ideanet-resources`** - File and resource management with linking strategies
6. **`ideanet-db`** - Database layer with SurrealDB embedded storage
7. **`ideanet-ui`** - React frontend with Chakra UI components

### Plugin Architecture
IdeaNet will support a WebAssembly-based plugin system for runtime extensibility. The plugin architecture enables users to discover and install plugins from a marketplace while the app is running. See [WASM Plugin System Architecture](../architecture/wasm-plugin-system.md) for detailed technical implementation.

Blocks and other extensible components are designed as independent plugins that can be developed separately and loaded dynamically using the WASM runtime.

---

## 7. User Stories (US-XXX)

*Note: Each module has its own detailed PRD with additional user stories:*
- *Audio Module: See [Audio Module PRD](./modules/audio.md) for US-AUDIO-001 through US-AUDIO-012*
- *AI Services Module: See [AI Services PRD](./modules/ai-services.md) for US-AI-001 through US-AI-015*
- *Resources Module: See [Resources Module PRD](./modules/resources.md) for US-RESOURCES-001 through US-RESOURCES-008*
- *Plugin System: See [WASM Plugin System Architecture](../architecture/wasm-plugin-system.md) for plugin implementation details*

### 7.1 Project Foundation and Setup (Core Module)

### 7.1.1 Initialize new IdeaNet project
- **ID**: US-001
- **Description**: As a developer, I want to set up a new Tauri project with React frontend so that I can start building the application from a solid foundation
- **Acceptance criteria**:
  - Tauri project scaffolding with React + TypeScript frontend
  - Windows (WinUI) and Android platform configuration
  - Basic development environment with hot reload and debugging tools
  - Project structure following DDD architecture principles

### 7.1.2 Establish database connection foundation
- **ID**: US-001.1
- **Description**: As a developer, I want to establish a basic database connection layer so that the application can reliably connect to and communicate with SurrealDB
- **Acceptance criteria**:
  - SurrealDB embedded database integration with RocksDB backend
  - Database connection management (connect/disconnect operations)
  - Health check functionality to verify database status
  - Proper error handling for connection failures
  - Database configuration management for different platforms

### 7.1.3 Implement clean architecture foundation
- **ID**: US-001.2  
- **Description**: As a developer, I want to implement DDD architecture layers so that the codebase is maintainable and supports future modular expansion
- **Acceptance criteria**:
  - Domain layer with error types and core abstractions
  - Application layer with service interfaces (ports) and business logic
  - Infrastructure layer with concrete database implementations
  - Tauri commands layer for frontend-backend communication
  - Clear separation of concerns following hexagonal architecture

### 7.1.4 Create database connection UI
- **ID**: US-001.3
- **Description**: As a developer, I want a simple UI to test database connectivity so that I can verify the foundation is working correctly
- **Acceptance criteria**:
  - React component displaying real-time database connection status
  - Connect/disconnect buttons for manual connection testing
  - Health check functionality with status indicators
  - Error message display for connection failures
  - Basic responsive design for different screen sizes

**Note**: User stories US-001.1 through US-001.3 represent the minimal scope for Phase 1 foundation implementation. All other features (business entities, CRUD operations, plugin system, event bus, testing framework) are explicitly deferred to future phases.

### 7.1.6 Configure database foundation (Phase 2)
- **ID**: US-002
- **Description**: As a developer, I want SurrealDB properly configured with schema migration system and robust connection management so that data persistence is reliable and can evolve with the application
- **Acceptance criteria**:
  - Database schema migration system for version updates and rollbacks
  - Database connection pooling and automatic error recovery
  - Database health monitoring and reconnection logic
  - Configuration management for different environments (dev, test, prod)
  - Database backup and restore functionality
  - Performance monitoring and query optimization foundation
  - Database transaction management patterns

### 7.1.7 Integrate UI framework foundation (Phase 2)
- **ID**: US-002.1
- **Description**: As a developer, I want to integrate Chakra UI component library so that the application has consistent, accessible styling from the beginning
- **Acceptance criteria**:
  - Chakra UI component library setup with consistent theming
  - Dark/light mode theme system with user preference persistence
  - Accessible UI components following WCAG guidelines
  - Theme customization for IdeaNet branding
  - Responsive design foundation for different screen sizes

### 7.2 Project Management (Phase 2-3)

### 7.2.1 Create and organize projects (Phase 3)
- **ID**: US-004
- **Description**: As a user, I want to create projects with descriptive metadata so that I can organize my ideas around specific goals and themes
- **Acceptance criteria**:
  - User can create project with required name and optional description
  - Projects support tagging for cross-cutting organization
  - Project status tracking (active, archived, completed, on-hold)
  - Project templates available for common use cases
  - Duplicate project detection and merging suggestions

### 7.2.2 Navigate project hierarchy (Phase 3)
- **ID**: US-005
- **Description**: As a user, I want to see my project structure clearly so that I can understand relationships and quickly access relevant content
- **Acceptance criteria**:
  - Visual project dashboard with statistics and recent activity
  - Hierarchical navigation with breadcrumbs
  - Quick access sidebar with pinned projects and favorites
  - Project search with metadata filtering
  - Recently accessed projects prominently displayed

### 7.2.3 Manage project lifecycle (Phase 3)
- **ID**: US-006
- **Description**: As a user, I want to track project progress and lifecycle stages so that I can maintain focus and measure advancement
- **Acceptance criteria**:
  - Project status workflow (concept → development → testing → complete)
  - Progress indicators based on completed milestones
  - Timeline view showing project evolution over time
  - Archive/restore functionality for completed projects
  - Bulk operations for project management

### 7.3 Rich Text Content Creation (Phase 4)

### 7.3.1 Integrate rich text editor
- **ID**: US-010.1
- **Description**: As a developer, I want to integrate BlockNote.js rich text editor so that users can create structured, extensible content blocks
- **Acceptance criteria**:
  - BlockNote.js integration with custom block support and extensibility
  - Basic text formatting (bold, italic, headers, lists, links)
  - Integration between BlockNote editor and Chakra UI design system
  - Block serialization and deserialization for database storage
  - Editor toolbar customization for IdeaNet-specific features

### 7.3.2 Create basic content blocks
- **ID**: US-011
- **Description**: As a user, I want to organize my ideas using different content block types so that I can structure information appropriately for different purposes
- **Acceptance criteria**:
  - Text block with rich formatting support
  - Heading blocks with multiple levels
  - List blocks (ordered and unordered)
  - Quote blocks for highlighting important information
  - Code blocks with syntax highlighting

### 7.3.3 Link and reference content
- **ID**: US-013
- **Description**: As a user, I want to create connections between content so that I can build a network of related ideas and information
- **Acceptance criteria**:
  - Internal linking between pages and blocks
  - Backlinks panel showing incoming references
  - Link suggestions based on content similarity
  - Visual indicators for linked content
  - Link validation and broken link detection

### 7.3.4 Establish future-ready architecture foundation
- **ID**: US-001.4
- **Description**: As a developer, I want the architecture to support future WASM plugin systems and multi-crate modules so that the foundation can scale without major refactoring
- **Acceptance criteria**:
  - Foundation for WASM-based plugin system that supports runtime loading
  - Service registry pattern for loose coupling between modules
  - Event bus foundation for plugin communication
  - Plugin marketplace integration foundation
  - Configuration system ready for plugin management
  - Documentation of WASM plugin architecture and security model

### 7.4 Audio Recording and Transcription (Phase 5)
*See [Audio Module PRD](./modules/audio.md) for detailed technical implementation*

### 7.4.1 Capture high-quality audio
- **ID**: US-007
- **Description**: As a user, I want to record clear audio from multiple sources so that I can capture ideas naturally through speech
- **Acceptance criteria**:
  - Microphone recording with device selection and level monitoring
  - System audio recording for capturing meeting audio or media
  - Simultaneous multi-source recording with separate tracks
  - Real-time audio level indicators and quality feedback
  - Background noise reduction and audio enhancement options

### 7.4.2 Process audio intelligently
- **ID**: US-008
- **Description**: As a user, I want my audio recordings automatically processed and transcribed so that they become searchable and actionable content
- **Acceptance criteria**:
  - Automatic transcription using OpenAI Whisper with word-level timestamps
  - Speaker diarization for multi-person recordings
  - Audio segmentation for large files to handle API limits
  - Confidence scoring for transcription accuracy
  - Manual transcription correction with learning feedback

### 7.4.3 Navigate audio content
- **ID**: US-009
- **Description**: As a user, I want to interact with audio recordings through their transcriptions so that I can quickly find and reference specific moments
- **Acceptance criteria**:
  - Click words in transcription to jump to audio timestamp
  - Visual highlighting of current word during playback
  - Playback speed control (0.5x to 2x speed)
  - A-B repeat functionality for focused listening
  - Audio bookmarking for important segments

### 7.4.4 Integrate audio with other content
- **ID**: US-010
- **Description**: As a user, I want to embed audio recordings and transcriptions into my project documentation so that spoken ideas become part of my structured thinking
- **Acceptance criteria**:
  - Audio block with embedded player and transcription
  - Export transcription text to other blocks or external formats
  - Link audio segments to related notes and resources
  - Audio summary generation using AI analysis
  - Collaborative commenting on specific audio timestamps

### 7.5 AI-Powered Assistance (Phase 6)
*See [AI Services Module PRD](./modules/ai-services.md) for detailed AI integration*

### 7.5.1 Generate contextual suggestions
- **ID**: US-015
- **Description**: As a user, I want to receive AI suggestions based on my current content so that I can discover new directions and overcome creative blocks
- **Acceptance criteria**:
  - Context-aware suggestions based on selected text or blocks
  - Multiple suggestion types (next steps, related ideas, questions, critiques)
  - Confidence scoring and ranking of suggestions
  - Suggestion history and ability to revisit previous ideas
  - User feedback loop to improve suggestion quality

### 7.5.2 Research and fact-checking assistance
- **ID**: US-016
- **Description**: As a user, I want AI help with research and verification so that I can build well-informed and credible projects
- **Acceptance criteria**:
  - Generate research questions for selected topics
  - Fact-checking with source citations and confidence levels
  - Alternative perspective generation for balanced analysis
  - Trend analysis and market research suggestions
  - Expert opinion synthesis from multiple sources

### 7.5.3 Create implementation roadmaps
- **ID**: US-017
- **Description**: As a user, I want AI assistance in turning abstract ideas into concrete action plans so that I can move from concept to execution
- **Acceptance criteria**:
  - Step-by-step implementation plan generation
  - Technology stack recommendations with rationale
  - Risk assessment and mitigation strategies
  - Timeline estimation with milestone planning
  - Resource requirement analysis and team skill mapping

### 7.5.4 Enhance content quality
- **ID**: US-018
- **Description**: As a user, I want AI help improving my writing and documentation so that I can communicate ideas more effectively
- **Acceptance criteria**:
  - Grammar and style improvement suggestions
  - Clarity and structure recommendations
  - Audience-appropriate tone and language adjustments
  - Summary generation for complex content
  - Presentation outline creation from project content

### 7.6 Resource Management (Phase 6)
*See [Resources Module PRD](./modules/resources.md) for detailed resource handling*

### 7.6.1 Link external resources flexibly
- **ID**: US-019
- **Description**: As a user, I want to connect external files and URLs to my projects so that I can maintain comprehensive context without duplicating content
- **Acceptance criteria**:
  - Soft linking with broken link detection and notifications
  - Deep copying for critical resources with version management
  - URL metadata extraction (title, description, preview image)
  - File type detection and appropriate preview generation
  - Batch import operations for multiple resources

### 7.6.2 Organize and search resources
- **ID**: US-020
- **Description**: As a user, I want to find and organize my project resources efficiently so that I can access relevant information quickly
- **Acceptance criteria**:
  - Resource categorization with tags and custom taxonomies
  - Full-text search across resource content and metadata
  - Visual thumbnail grid for images and documents
  - Resource usage tracking and related content suggestions
  - Duplicate detection and consolidation recommendations

### 7.6.3 Embed rich media content
- **ID**: US-021
- **Description**: As a user, I want to embed various media types directly in my notes so that I can create rich, multimedia documentation
- **Acceptance criteria**:
  - Image embedding with annotation and captioning
  - Video embedding with timestamp linking
  - PDF viewer with highlight and note capabilities
  - Interactive charts and data visualizations
  - Code snippet execution and syntax highlighting

### 7.6.4 Manage storage efficiently
- **ID**: US-022
- **Description**: As a user, I want to understand and control my storage usage so that I can maintain optimal performance and cost
- **Acceptance criteria**:
  - Storage usage dashboard with breakdown by project and resource type
  - Compression options for large files
  - Archive/cleanup suggestions for unused resources
  - Storage quota warnings and management tools
  - Export/backup options for data portability

### 7.7 Search and Discovery (Phase 6-7)

### 7.7.1 Search across all content
- **ID**: US-023
- **Description**: As a user, I want to search across all my projects and content types so that I can find relevant information regardless of where it's stored
- **Acceptance criteria**:
  - Global search across projects, notes, resources, and transcriptions
  - Search result ranking by relevance, recency, and user interaction patterns
  - Advanced filtering by content type, date range, tags, and project
  - Search suggestions and autocomplete based on content and history
  - Saved searches and search alerts for new matching content

### 7.7.2 Discover content relationships
- **ID**: US-024
- **Description**: As a user, I want to discover connections between my content so that I can identify patterns and build on previous work
- **Acceptance criteria**:
  - Semantic similarity search using AI embeddings
  - "Find similar" functionality for any content item
  - Content clustering and theme identification
  - Timeline view showing content evolution and relationships
  - Recommendation engine for related projects and ideas

### 7.7.3 Handle empty states gracefully
- **ID**: US-025
- **Description**: As a user, I want to see helpful guidance when areas are empty so that I understand how to get started and make progress
- **Acceptance criteria**:
  - Welcome flow for new users with guided project creation
  - Empty project states with suggestions for first steps
  - Empty search results with alternative query suggestions
  - Onboarding tooltips and progressive disclosure of features
  - Template suggestions based on user goals and project type

### 7.8 WASM Plugin System (Phase 7-8)
*See [WASM Plugin System Architecture](../architecture/wasm-plugin-system.md) for detailed implementation*

### 7.8.1 Establish module interfaces and event bus
- **ID**: US-003
- **Description**: As a developer, I want clear interfaces between modules so that they can be developed independently while maintaining system coherence
- **Acceptance criteria**:
  - Event bus implementation for loose coupling between modules
  - Plugin registry system for WASM module loading
  - Error handling patterns consistent across all modules
  - Module initialization and dependency injection system
  - Integration test framework for module interactions

### 7.8.2 Implement WASM plugin runtime
- **ID**: US-014
- **Description**: As a developer, I want to create a WASM plugin runtime so that users can install and run plugins securely
- **Acceptance criteria**:
  - WASM plugin loader with sandbox security model
  - Plugin capability system for permission management
  - Plugin lifecycle management (load, initialize, shutdown, unload)
  - Inter-plugin communication through event bus
  - Plugin hot-reload support for development

### 7.8.3 Create plugin marketplace integration
- **ID**: US-030
- **Description**: As a user, I want to discover and install plugins from a marketplace so that I can extend the application's functionality
- **Acceptance criteria**:
  - Plugin store with search and discovery functionality
  - One-click plugin installation and updates
  - Plugin ratings, reviews, and usage statistics
  - Plugin signature verification for security
  - Installed plugin management interface

### 7.8.4 Develop plugin SDK and tooling
- **ID**: US-031
- **Description**: As a plugin developer, I want development tools and SDK so that I can create plugins efficiently
- **Acceptance criteria**:
  - Plugin development kit with Rust and TypeScript support
  - Plugin template and scaffolding tools
  - Local plugin testing and debugging tools
  - Plugin publishing and deployment tools
  - Comprehensive documentation and examples

### 7.9 Export and Sharing (Phase 8)

### 7.9.1 Export projects in multiple formats
- **ID**: US-026
- **Description**: As a user, I want to export my projects in various formats so that I can share them appropriately with different audiences
- **Acceptance criteria**:
  - PDF export with embedded media and clickable links
  - Markdown export with linked resources and standard formatting
  - HTML export for web publishing with responsive design
  - Presentation mode with slides generated from project structure
  - JSON export for data portability and integration

### 7.9.2 Create shareable project views
- **ID**: US-027
- **Description**: As a user, I want to create focused views of my projects so that I can share relevant information without exposing sensitive details
- **Acceptance criteria**:
  - Custom view creation with filtered content
  - Permission-based sharing with read-only access
  - Public link generation with expiration dates
  - Embedded view widgets for external websites
  - Analytics on shared content engagement

### 7.10 Collaboration Features (Phase 8)

### 7.10.1 Comment and annotate content
- **ID**: US-028
- **Description**: As a collaborator, I want to add comments and annotations to project content so that I can provide feedback and suggestions
- **Acceptance criteria**:
  - Inline comments on text selections and blocks
  - Annotation tools for images and documents
  - Comment threading and resolution tracking
  - @mention notifications for team members
  - Comment export and archive capabilities

### 7.10.2 Track changes and versions
- **ID**: US-029
- **Description**: As a user, I want to see how content has changed over time so that I can understand the evolution of ideas and revert if needed
- **Acceptance criteria**:
  - Automatic version history for all content changes
  - Visual diff view showing additions, deletions, and modifications
  - Version comparison between any two points in time
  - Selective revert functionality for specific changes
  - Change attribution and timestamping

---

## 8. Technical Implementation Priorities

### Phase 1: Foundation Only (Current Scope - Weeks 1-2)
- **US-001**: Tauri project setup with React + TypeScript frontend
- **US-001.1**: Database connection foundation with SurrealDB
- **US-001.2**: Clean DDD architecture implementation  
- **US-001.3**: Basic database connection UI for testing
- **Deliverable**: Working database connectivity with health checking UI

**Explicitly Out of Scope for Phase 1:**
- ❌ Business entities, CRUD operations, schema migrations
- ❌ Plugin system, event bus, module interfaces  
- ❌ Audio, AI, blocks, or resource management features
- ❌ Unit/integration testing (deferred to Phase 2)
- ❌ BlockNote.js, Chakra UI integration

### Phase 2: Core Infrastructure (Weeks 3-6)
- **US-002**: Database infrastructure (migrations, pooling, monitoring, transactions)
- **US-002.1**: Chakra UI component library integration and theming
- **Testing framework**: Unit and integration test implementation
- **Deliverable**: Robust database infrastructure with consistent UI foundation

### Phase 3: Project Management (Weeks 7-10)  
- **US-004, US-005, US-006**: Complete project management features
- **US-023**: Basic search functionality
- **Deliverable**: Full project CRUD operations with search

### Phase 4: Rich Text Content Creation (Weeks 11-14)
- **US-010.1**: BlockNote.js rich text editor integration
- **US-011**: Basic content blocks (text, headers, lists, quotes, code)
- **US-013**: Internal linking and references
- **US-001.4**: Future-ready architecture foundation with WASM plugin system design
- **Deliverable**: Rich content creation with linking capabilities and plugin architecture foundation

### Phase 5: Audio Integration (Weeks 15-18)
- **US-007, US-008**: Audio recording and transcription
- **US-009, US-010**: Audio navigation and integration
- **Audio module**: Complete implementation with Whisper integration
- **Deliverable**: Full audio-to-text workflow integrated with content blocks

### Phase 6: AI Integration & Resources (Weeks 19-22)
- **US-015, US-016**: AI suggestions and research assistance
- **US-017, US-018**: Implementation planning and content enhancement
- **US-019, US-020**: Resource linking and organization
- **US-021, US-022**: Rich media and storage management
- **Deliverable**: Intelligent content assistance with resource management

### Phase 7: Advanced Features & Plugin Foundation (Weeks 23-26)
- **US-024, US-025**: Content discovery and empty states
- **US-003**: Event bus and module interface implementation
- **US-014**: WASM plugin runtime with security model
- **Deliverable**: Plugin runtime ready for marketplace integration

### Phase 8: Plugin Marketplace & Collaboration (Weeks 27-30)
- **US-030**: Plugin marketplace integration with discovery and installation
- **US-031**: Plugin SDK and development tooling
- **US-026, US-027**: Export and sharing capabilities
- **US-028, US-029**: Collaboration features
- **Deliverable**: Production-ready application with full plugin ecosystem
